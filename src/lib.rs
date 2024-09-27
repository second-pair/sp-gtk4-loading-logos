/*  *--<Preface>--*  //

 -=-  Author Details  -=-
 Blair Edwards
 My own shenanigans.

 -=-  Dates  -=-
 Started 2024-09-09

 -=-  Description  -=-
 This library provides an API for creating logo-style loading spinners.
 IO is through API calls via a C-style header and archive.
 Config is through compile-time 'const's and API calls.

 -=-  Task  -=-
 -=>

 -=-  Notes  -=-
 -=>  I've developed my own commenting notation for things that "aren't done" one way or another.  Such as:
	 -  //#  TODO
	 -  //?  Not sure / query
	 -  //!  Important note / relevant as technology advances
 -=>  Logging with `_LOG ()` takes a 'logLevel' argument, which roughly indicates:
	 -  0:  Critical, major errors, should Always be printed.
	 -  1:	Important info particularly critical functions, minor / user errors.
	 -  2:	Useful info / general programme flow.
	 -  3:	Debug info, steps throughout a function.
	 -  4:  Useful spam - printed often such as in a loop.
	 -  5:  Debug spam - printed often such as in a loop.
 -=>

//  *--</Preface>--*  */



//  *--<Preparations>--*  //

//  Local Compiler Pragmas
#! [allow (unused_variables)]
#! [allow (non_snake_case)]
#! [allow (unused_parens)]
//#! [allow (dead_code)]
//#! [allow (unused_imports)]

//  Imports
//use std ::io ::{stdin, stdout};
//use std ::io ::Result;
//use std ::time;
//use std ::fs ::File;
//use std ::path ::Path;
//use std ::io ::{Read, BufReader, BufRead};
//use std ::io ::{Write, BufWriter};
//use std ::rc ::Rc;
//use std ::cell ::RefCell;
//  of Which are GTK4
use gtk4 as gtk;
use gtk ::prelude ::*;
use glib ::Object;
use gtk ::glib;
use gtk ::glib ::ControlFlow;
use gtk ::glib ::clone;
//  of Which are Local
mod logo_types;
use crate ::logo_types ::LogoType;

//  Global Enumerations

//  Global Constants
const TIME_ANIM: u16 = 50;

//  Global Variables

//  Local Constants

//  Local Variables

//  Structures

//  *--</Preparations>--*  //



//  *--<Macros>--*  //

//  *--</Macros>--*  //



//  *--<Traits & Implementations>--*  //

//  Create the public-facing structure and apply the 'glib' wrapper to encase it as a GObject.
glib ::wrapper!
{
	pub struct LoadingLogo (ObjectSubclass <logo_impl ::LoadingLogo>)
	@extends gtk ::DrawingArea, gtk ::Widget,
	@implements gtk ::Accessible, gtk ::Actionable, gtk ::Buildable, gtk::ConstraintTarget;
}

//  Implement the public-facing API for this structure.
impl LoadingLogo
{
	pub fn create (test_number: i32) -> Self
	{
		let da_logo: LoadingLogo = Object ::builder ()
			.property ("test_number", test_number)
			.build ();
		da_logo .set_hexpand (true);
		da_logo .set_vexpand (true);
		//da_logo .test_number = 14;

		//  Add a timeout to update the logo's positions.
		//  Local, so we don't mess with GTK's main-thread requirements.
		gtk ::glib ::timeout_add_local
		(
			//core ::time ::Duration ::from_millis (TIME_ANIM as u64),
			core ::time ::Duration ::from_millis (TIME_ANIM as u64),
			clone! (#[strong] da_logo, move ||
			{
				da_logo .queue_draw ();
				return ControlFlow ::Continue;
			}
		));

		return da_logo;
	}

	pub fn retrieve_number (&self) -> i32
	{
		//return self .test_number ();
		return 7;
	}
}

//  *--</Traits & Implementations>--*  //



//  *--<Main Code>--*  //

//  C API.
# [no_mangle]
pub unsafe extern "C" fn sp_gtk4_loading_logos_create () -> gtk ::Widget
{
	gtk ::init () .unwrap ();

	//return priv_create ();
	let newLogo = LoadingLogo ::create (17);
	//newLogo .anim_type () .to_number ();
	print! ("test_number:  {}\n", newLogo .retrieve_number ());
	return newLogo .into ();
}

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

//  *--</Callbacks>--*  //


mod logo_impl
{
	use gtk4 as gtk;
	use gtk ::prelude ::*;
	use gtk ::glib;
	use gtk ::glib ::clone;
	use gtk ::glib ::Properties;
	use gtk ::subclass ::prelude ::*;

	use crate ::logo_types ::LogoType;

	const DRAW_TARGET_LEN: f64 = 1000.0;
	const DRAW_LINE_WIDTH_BASE: f64 = 10.0;

	# [derive (Properties, Default)]
	# [properties (wrapper_type = super ::LoadingLogo)]
	pub struct LoadingLogo
	{
		# [property (get, set)]
		test_number: std ::cell ::Cell<i32>,
		anim_type: std ::cell ::Cell <LogoType>,
		iter: std ::cell ::Cell <f64>,
	}

	# [glib ::object_subclass]
	impl ObjectSubclass for LoadingLogo
	{
		const NAME:  &'static str = "LoadingLogo";
		type Type = super ::LoadingLogo;
		type ParentType = gtk ::DrawingArea;
	}
	//  Trait shared by all GObjects
	# [glib::derived_properties]
	impl ObjectImpl for LoadingLogo
	{
		fn constructed (&self)
		{
			self .parent_constructed ();

			//

			//  Set up the draw function.
			# [allow (deprecated)]
			DrawingAreaExtManual ::set_draw_func (self .obj () .as_ref (),
				clone! (@weak self as widget => move |_, cairo, width, height|
			{
				//  Draw a box outline to help suss out the widget's area.
				cairo .rectangle (0.0, 0.0, width as f64, height as f64);

				//  Scale factor - calculated from 'width' and 'height'.
				let areaScale = core ::cmp ::min (width, height) as f64 / DRAW_TARGET_LEN;

				//  Move the origin to the middle and flip the Y-axis.
				let matrix = gtk ::cairo ::Matrix ::new (1.0, 0.0, 0.0, -1.0, width as f64 / 2.0, height as f64 / 2.0);
				cairo .transform (matrix);

				//  Perform the draw.
				let anim_type = widget .anim_type .get ();
				anim_type .draw (cairo, widget .iter .get (), areaScale);
				//  Iterate the iterator.
				widget .iter .set (widget .iter .get () + 1.0);

				//  Render that line.
				cairo .set_line_width (DRAW_LINE_WIDTH_BASE * areaScale);
				cairo .set_line_cap (gtk ::cairo ::LineCap ::Round);
				cairo .set_line_join (gtk ::cairo ::LineJoin ::Round);
				cairo .set_source_rgba (1.0, 1.0, 1.0, 1.0);
				cairo .stroke () .unwrap ();
			}));
		}
	}
	//  Trait shared by all widgets
	impl WidgetImpl for LoadingLogo {}
	//  Trait shared by all drawing areas
	impl DrawingAreaImpl for LoadingLogo {}
}
