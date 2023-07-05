pub extern "C" fn Servo_GetComputedKeyframeValues(
    keyframes: &nsTArray<structs::Keyframe>,
    element: &RawGeckoElement,
    pseudo_type: PseudoStyleType,
    style: &ComputedValues,
    raw_data: &PerDocumentStyleData,
    computed_keyframes: &mut nsTArray<structs::ComputedKeyframeValues>,
) {
    let data = raw_data.borrow();
    let element = GeckoElement(element);
    let pseudo = PseudoElement::from_pseudo_type(pseudo_type);
    let parent_element = if pseudo.is_none() {
        element.inheritance_parent()
    } else {
        Some(element)
    };
    let parent_data = parent_element.as_ref().and_then(|e| e.borrow_data());
    let parent_style = parent_data
        .as_ref()
        .map(|d| d.styles.primary())
        .map(|x| &**x);

    let container_size_query =
        ContainerSizeQuery::for_element(element, pseudo.as_ref().and(parent_style));
    let mut conditions = Default::default();
    let mut context = create_context_for_animation(
        &data,
        &style,
        parent_style,
        /* for_smil_animation = */ false,
        &mut conditions,
        container_size_query,
    );

    let restriction = pseudo.and_then(|p| p.property_restriction());

    let global_style_data = &*GLOBAL_STYLE_DATA;
    let guard = global_style_data.shared_lock.read();
    let default_values = data.default_computed_values();

    let mut raw_custom_properties_block; // To make the raw block alive in the scope.
    for (index, keyframe) in keyframes.iter().enumerate() {
        let mut custom_properties = None;
        for property in keyframe.mPropertyValues.iter() {
            // Find the block for custom properties first.
            if property.mProperty == nsCSSPropertyID::eCSSPropertyExtra_variable {
                raw_custom_properties_block = unsafe { &*property.mServoDeclarationBlock.mRawPtr };
                let guard = raw_custom_properties_block.read_with(&guard);
                custom_properties = guard.cascade_custom_properties_with_context(&context);
                // There should be one PropertyDeclarationBlock for custom properties.
                break;
            }
        }

        let ref mut animation_values = computed_keyframes[index];

        let mut seen = LonghandIdSet::new();

        let mut property_index = 0;
        for property in PrioritizedPropertyIter::new(&keyframe.mPropertyValues) {
            if simulate_compute_values_failure(property) {
                continue;
            }

            let mut maybe_append_animation_value =
                |property: LonghandId, value: Option<AnimationValue>| {
                    debug_assert!(!property.is_logical());
                    debug_assert!(property.is_animatable());

                    // 'display' is only animatable from SMIL
                    if property == LonghandId::Display {
                        return;
                    }

                    // Skip restricted properties
                    if restriction.map_or(false, |r| !property.flags().contains(r)) {
                        return;
                    }

                    if seen.contains(property) {
                        return;
                    }
                    seen.insert(property);

                    // This is safe since we immediately write to the uninitialized values.
                    unsafe {
                        animation_values.set_len((property_index + 1) as u32);
                        ptr::write(
                            &mut animation_values[property_index],
                            structs::PropertyStyleAnimationValuePair {
                                mProperty: property.to_nscsspropertyid(),
                                mValue: structs::AnimationValue {
                                    mServo: value.map_or(structs::RefPtr::null(), |v| {
                                        structs::RefPtr::from_arc(Arc::new(v))
                                    }),
                                },
                            },
                        );
                    }
                    property_index += 1;
                };

            if property.mServoDeclarationBlock.mRawPtr.is_null() {
                let property = LonghandId::from_nscsspropertyid(property.mProperty);
                if let Ok(prop) = property {
                    maybe_append_animation_value(prop, None);
                }
                continue;
            }

            let declarations = unsafe { &*property.mServoDeclarationBlock.mRawPtr };
            let guard = declarations.read_with(&guard);
            let iter = guard.to_animation_value_iter(
                &mut context,
                &default_values,
                custom_properties.as_ref(),
            );

            for value in iter {
                let id = value.id();
                maybe_append_animation_value(id, Some(value));
            }
        }
    }
}

// https://github.com/mozilla/gecko-dev/blob/9a4666e63199bd1bcfc9095f6efec3488c358458/servo/ports/geckolib/glue.rs#L6147