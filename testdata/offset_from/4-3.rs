fn merge_sorted_cursors(&mut self) {
    fn to_cursor(range: BufferRange, forward: bool) -> Cursor {
        if forward {
            Cursor {
                anchor: range.from,
                position: range.to,
            }
        } else {
            Cursor {
                anchor: range.to,
                position: range.from,
            }
        }
    }

    let ptr_range = self.cursors.as_mut_ptr_range();
    let start_ptr = ptr_range.start;
    let end_ptr = ptr_range.end;

    let mut write_ptr = start_ptr;
    let mut read_ptr = unsafe { write_ptr.add(1) };
    let mut write_i = 0;

    let (mut range, mut forward) = unsafe { write_ptr.read() }.to_range_and_direction();
    while read_ptr != end_ptr {
        let other_cursor = unsafe { read_ptr.read() };
        let (other_range, other_forward) = other_cursor.to_range_and_direction();

        if other_range.from <= range.to {
            if write_i < self.main_cursor_index as _ {
                self.main_cursor_index -= 1;
            }

            range.to = range.to.max(other_range.to);
        } else {
            let cursor = to_cursor(range, forward);
            let store_ptr = write_ptr;
            write_i += 1;

            range = other_range;
            forward = other_forward;

            unsafe { write_ptr = write_ptr.add(1) };
            unsafe { store_ptr.write(cursor) };
        }

        unsafe { read_ptr = read_ptr.add(1) };
    }

    let cursor = to_cursor(range, forward);
    unsafe { write_ptr.write(cursor) };

    let len = unsafe { write_ptr.add(1).offset_from(start_ptr) as _ };
    self.cursors.truncate(len);
}
//https://github.com/vamolessa/pepper/blob/10955408464a4c5958f2c32ee7e88607ce03b20a/pepper/src/cursor.rs#L145