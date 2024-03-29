use std;
use std::ops::Range;

pub struct GapBuffer<T> {
    // Storage for elements. This has the capacity we need, but its length
    // always remains zero. GapBuffer puts its elements and the gap in this
    // `Vec`'s "unused" capacity.
    storage: Vec<T>,

    // Range of uninitialized elements in the middle of `storage`.
    // Elements before and after this range are always initialized.
    gap: Range<usize>
}

impl<T> GapBuffer<T> {
    pub fn new() -> GapBuffer<T> {
        GapBuffer { storage: Vec::new(), gap: 0..0 }
    }

    /// Return the number of elements this GapBuffer could hold without
    /// reallocation.
    pub fn capacity(&self) -> usize {
        self.storage.capacity()
    }

    /// Return the number of elements this GapBuffer currently holds.
    pub fn len(&self) -> usize {
        self.capacity() - self.gap.len()
    }

    /// Return the current insertion position.
    pub fn position(&self) -> usize {
        self.gap.start
    }

    /// Return a pointer to the `index`th element of the underlying storage,
    /// regardless of the gap.
    ///
    /// Safety: `index` must be a valid index into `self.storage`.
    unsafe fn space(&self, index: usize) -> *const T {
        self.storage.as_ptr().offset(index as isize)
    }

    /// Return a mutable pointer to the `index`th element of the underlying
    /// stoarge, regardless of the gap.
    ///
    /// Safety: `index` must be a valid index into `self.storage`.
    unsafe fn space_mut(&mut self, index: usize) -> *mut T {
        self.storage.as_mut_ptr().offset(index as isize)
    }

    /// Return the offset in the buffer of the `index`th element, taking
    /// the gap into account. This does not check whether index is in range,
    /// but it never returns an index in the gap.
    fn index_to_raw(&self, index: usize) -> usize {
        if index < self.gap.start {
            index
        } else {
            index + self.gap.len()
        }
    }

    /// Return a reference to the `index`th element,
    /// or `None` if `index` is out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        let raw = self.index_to_raw(index);
        if raw < self.capacity() {
            unsafe {
                // We just checked `raw` against self.capacity(),
                // and index_to_raw skips the gap, so this is safe.
                Some(&*self.space(raw))
            }
        } else {
            None
        }
    }

    /// Set the current insertion position to `pos`.
    /// If `pos` is out of bounds, panic.
    pub fn set_position(&mut self, pos: usize) {
        if pos > self.len() {
            panic!("index {} out of range for GapBuffer", pos);
        }

        unsafe {
            let gap = self.gap.clone();
            if pos > gap.start {
                // `pos` falls after the gap. Move the gap right
                // by shifting elements after the gap to before it.
                let distance = pos - gap.start;
                // pub const unsafe fn copy<T>(src: *const T, dst: *mut T, 
                // count: usize)
                std::ptr::copy(self.space(gap.end),
                               self.space_mut(gap.start),
                               distance);
            } else if pos < gap.start {
                // `pos` falls before the gap. Move the gap left
                // by shifting elements before the gap to after it.
                let distance = gap.start - pos;
                // pub const unsafe fn copy<T>(src: *const T, dst: *mut T, 
                // count: usize)
                std::ptr::copy(self.space(pos),
                               self.space_mut(gap.end - distance),
                               distance);
            }

            self.gap = pos .. pos + gap.len();
        }
    }

    /// Insert `elt` at the current insertion position,
    /// and leave the insertion position after it.
    pub fn insert(&mut self, elt: T) {
        // if the gap has been filled in,
        // the enlarge_gap method double the capacity of
        // `self.storage`
        if self.gap.len() == 0 {
            self.enlarge_gap();
        }

        unsafe {
            let index = self.gap.start;
            std::ptr::write(self.space_mut(index), elt);
        }
        self.gap.start += 1;
        println!("after insert , self.gap.start:{:?}, self.gap.end:{:?}", 
            self.gap.start, self.gap.end);
    }

    /// Insert the elements produced by `iter` at the current insertion
    /// position, and leave the insertion position after them.
    pub fn insert_iter<I>(&mut self, iterable: I)
        where I: IntoIterator<Item=T>
    {
        for item in iterable {
            self.insert(item)
        }
    }

    /// Remove the element just after the insertion position
    /// and return it, or return `None` if the insertion position
    /// is at the end of the GapBuffer.
    pub fn remove(&mut self) -> Option<T> {
        if self.gap.end == self.capacity() {
            return None;
        }

        let element = unsafe {
            // whereas removal moves one value out 
            std::ptr::read(self.space(self.gap.end))
                
        };
        // and enlarges the gap to cover the space it used to occupy
        self.gap.end += 1;
        Some(element)
    }

    /// Double the capacity of `self.storage`.
    fn enlarge_gap(&mut self) {
        let mut new_capacity = self.capacity() * 2;
        if new_capacity == 0 {
            // The existing vector is empty.
            // Choose a reasonable starting capacity.
            new_capacity = 4;
        }

        // We have no idea what resizing a Vec does with its "unused"
        // capacity. So just create a new vector and move over the elements.
        let mut new = Vec::with_capacity(new_capacity);
        let after_gap = self.capacity() - self.gap.end;
        let new_gap = self.gap.start .. new.capacity() - after_gap;
        
        unsafe {
            // Move the elements that fall before the gap.
            std::ptr::copy_nonoverlapping(self.space(0),
                                          new.as_mut_ptr(),
                                          self.gap.start);

            // Move the elements that fall after the gap.
            let new_gap_end = new.as_mut_ptr().offset(new_gap.end as isize);
            std::ptr::copy_nonoverlapping(self.space(self.gap.end),
                                          new_gap_end,
                                          after_gap);
        }

        // This frees the old Vec, but drops no elements,
        // because the Vec's length is zero.
        self.storage = new;
        self.gap = new_gap;
    }


}


pub struct Iter<'a, T> {
    buffer: &'a GapBuffer<T>,
    pos: usize
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<&'a T> {
        if self.pos >= self.buffer.len() {
            None
        } else {
            self.pos += 1;
            self.buffer.get(self.pos - 1)
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a GapBuffer<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Iter<'a, T> {
        Iter { buffer: self, pos: 0 }
    }
}

impl GapBuffer<char> {
    pub fn get_string(&self) -> String {
        let mut text = String::new();
        text.extend(self);
        text
    }

}


use std::fmt;
impl<T: fmt::Debug> fmt::Debug for GapBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let indices = (0..self.gap.start).chain(self.gap.end .. self.capacity());
        let elements = indices.map(|i| unsafe { &*self.space(i) });
        f.debug_list().entries(elements).finish()
    }
}


impl<T> Drop for GapBuffer<T> {
    fn drop(&mut self) {
        unsafe {
            for i in 0 .. self.gap.start {
                std::ptr::drop_in_place(self.space_mut(i));
            }
            for i in self.gap.end .. self.capacity() {
                std::ptr::drop_in_place(self.space_mut(i));
            }
        }
    }
}



#[test]
fn test_gapbuffer() {
    let mut buf = GapBuffer::new();
    println!("after GapBuffer::new, buf.gap.start:{:?}, buf.gap.end:{:?}", buf.gap.start, buf.gap.end);
    buf.insert_iter("Lord of the Rings".chars());
    println!("after insert_iter, buf.gap.start:{:?}, buf.gap.end:{:?}", buf.gap.start, buf.gap.end);
    buf.set_position(12);
    println!("after set_position(12), buf.gap.start:{:?}, buf.gap.end:{:?}", buf.gap.start, buf.gap.end);
    buf.insert_iter("Onion ".chars());
    println!("after insert_iter, buf.gap.start:{:?}, buf.gap.end:{:?}", buf.gap.start, buf.gap.end);
    assert_eq!(buf.get_string(), "Lord of the Onion Rings");
}


