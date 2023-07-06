unsafe fn drop_state(&self, place: StateAddr) {
    let state = place.get::<StringAggState>();
    std::ptr::drop_in_place(state);
}

// https://github.com/datafuselabs/databend/blob/21dd947a11ea8c46614198f91cbff0bba517a280/src/query/functions/src/aggregates/aggregate_string_agg.rs#L159