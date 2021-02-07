pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }
}

#[no_mangle]
pub extern "C" fn new_point(x: i32, y: i32) -> *mut Point {
    Box::into_raw(Box::new(Point::new(x, y)))
}

#[no_mangle]
pub extern "C" fn delete_point(p: *mut Point) {
    if !p.is_null() {
        unsafe {
            Box::from_raw(p);
        }
    }
}

#[no_mangle]
pub extern "C" fn get_x(p: *const Point) -> i32 {
    unsafe {
        p.as_ref().expect("invalid pointer").get_x()
    }
}

#[no_mangle]
pub extern "C" fn get_y(p: *const Point) -> i32 {
    unsafe {
        p.as_ref().expect("invalid pointer").get_y()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
