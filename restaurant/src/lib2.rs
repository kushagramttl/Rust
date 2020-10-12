/**
 *
 * In this example, we can see how the scope rule works
 * To access the method in parent scope, we can use the super keyword
 * That way we will be accessing the method in parent scope.
 *
 */
fn serve_order() {}

mod back_of_house_2 {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
