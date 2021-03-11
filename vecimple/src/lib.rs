#[macro_export]

macro_rules! avec {
   ($($element: expr),* $(,)?) => {{
       const C: usize = $crate::count![@COUNT; $($element),*];
       #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(C);
        $(vs.push($element);)* 
        vs
    }};

   ($element: expr; $count:expr) => {{
        let mut vs = Vec::new();
        vs.resize($count, $element);
        vs
    }};

}

#[macro_export]
#[doc(hidden)]
macro_rules! count {

   (@COUNT; $($element:expr),*) => {
        <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
   };
    
   (@SUBST; $_element:expr) => { () };
}
#[test]
fn empty_vec() {
    let x: Vec<u32> = avec![];
    assert!(x.is_empty());
}


#[test]
fn single() {
    let x: Vec<u32> = avec![42];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double() {
    let x: Vec<u32> = avec![42, 43];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 43);
}

#[test]
fn trailing() {
    let x: Vec<&'static str> = avec!["helloworld",];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], "helloworld");
}

#[test]
fn clone_2() {
    let x: Vec<u32> = avec![42;2];
    assert!(!x.is_empty());
    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 42);
}
