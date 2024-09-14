//#  I should have the unsafe functions call safe functions.  Maybe even abstract them to a different module?  Or file?

//  of Which are GTK4
use gtk4 as gtk;
use gtk ::prelude ::*;
use gtk ::glib;
use gtk ::glib ::clone;

# [no_mangle]
pub unsafe extern "C" fn sp_gtk4_loading_logos_create () -> gtk ::Label
{
    let label_logo = gtk ::Label ::builder ()
		//.text ("Hello from the library!")
		.build ();
	label_logo .set_text ("Hello from the library!");
	return label_logo;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
