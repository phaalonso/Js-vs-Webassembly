mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn run() {
    // Seta  o console_error_panic_hook para obeter erros mais legiveis
    utils::set_panic_hook();
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many3(a: &str, b: &str, c: &str);

}

#[wasm_bindgen]
pub fn testeaa(array: &mut [i32], start: isize, end: isize) {
    quick_sort(array, start, end);
    // array.sort();

    for elem in array {
        log(&elem.to_string());
        
    }
}

#[wasm_bindgen]
pub fn quick_sort(array: &mut [i32], start: isize, end: isize) {
    // log_many3("Quick sort", &start.to_string(), &end.to_string());
    if start < end {
        let pivot = partition(array, start, end);
        // log_many("Pivo: ",&pivot.to_string());
    
        quick_sort(array, start, pivot - 1);
        quick_sort(array, pivot + 1, end);
    }
}

pub fn partition(array: &mut [i32], start: isize, end: isize ) -> isize {
    // log_many3("partition", &start.to_string(), &end.to_string());
    let pivot = array[end as usize];
    let mut i = start - 1;

    for j in start..end {
       if array[j as usize] <= pivot {
           i += 1;
           array.swap(i as usize, j as usize);
       }
    }
    array.swap((i as usize) +1, end as usize);
 
    // log("end partition");
    return i +1;
}