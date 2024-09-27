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
use crate ::logo_types ::LtType;

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
	pub fn create (anim_type: Option <LtType>) -> Self
	{
		let anim_type = match (anim_type)
		{
			None => LogoType ::default_value (),
			Some (val) => LogoType ::from_value_or_default (val) .to_value (),
		};
		let da_logo: LoadingLogo = Object ::builder ()
			.property ("anim_type", anim_type)
			.build ();
		da_logo .set_hexpand (true);
		da_logo .set_vexpand (true);

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
}

//  *--</Traits & Implementations>--*  //



//  *--<Main Code>--*  //

//  C API.
# [no_mangle]
pub extern "C" fn sp_gtk4_loading_logos_create_default () -> gtk ::Widget
{
	return sp_gtk4_loading_logos_create (LogoType ::default_value ());
}
# [no_mangle]
pub extern "C" fn sp_gtk4_loading_logos_create (anim_type: LtType) -> gtk ::Widget
{
	gtk ::init () .unwrap ();

	let newLogo = LoadingLogo ::create (Some (anim_type));
	return newLogo .into ();
}
# [no_mangle]
pub extern "C" fn sp_gtk4_loading_logos_get_type (logo: LoadingLogo) -> LtType
{
	//  Prevent dropping 'logo' at the end of the function.
	let logo = Box ::leak (Box ::new (logo));
	return logo .anim_type ();
}
# [no_mangle]
pub extern "C" fn sp_gtk4_loading_logos_set_type (logo: LoadingLogo, anim_type: LtType)
{
	//  Prevent dropping 'logo' at the end of the function.
	let logo = Box ::leak (Box ::new (logo));
	logo .set_anim_type (anim_type);
}
# [no_mangle]
pub extern "C" fn sp_gtk4_loading_logos_max_type () -> LtType
{
	return LogoType ::max_value ();
}

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

//  *--</Callbacks>--*  //


mod logo_impl
{
	use std ::f64 ::consts ::PI;

	use gtk4 as gtk;
	use gtk ::prelude ::*;
	use gtk ::glib;
	use gtk ::glib ::clone;
	use gtk ::glib ::Properties;
	use gtk ::subclass ::prelude ::*;

	use crate ::logo_types ::LogoType;
	use crate ::logo_types ::LtType;

	const DRAW_TARGET_LEN: f64 = 0.8;
	const DRAW_LINE_WIDTH_BASE: f64 = 0.014;
	const DRAW_RAD_BASE: f64 = 0.2;

	# [derive (Properties, Default)]
	# [properties (wrapper_type = super ::LoadingLogo)]
	pub struct LoadingLogo
	{
		# [property (get, set)]
		anim_type: std ::cell ::Cell<LtType>,
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
				let wF64 = width as f64;
				let hF64 = height as f64;
				let areaLen = core ::cmp ::min (width, height) as f64;

				//  Scale dimension (in pixels) - calculated from 'width' and 'height'.  We'll shrink this a bit further shortly.
				let areaScale = areaLen * DRAW_TARGET_LEN;
				let borderLen = areaLen / 2.0;
				let borderRad = areaScale * DRAW_RAD_BASE;

				//  Move the origin to the middle and flip the Y-axis.
				let matrix = gtk ::cairo ::Matrix ::new (1.0, 0.0, 0.0, -1.0, wF64 / 2.0, hF64 / 2.0);
				cairo .transform (matrix);

				//  Draw a rounded rectangle to bound the logo.
				cairo .move_to (-borderLen, borderLen - borderRad);
				cairo .line_to (-borderLen, -borderLen + borderRad);
				cairo .arc (-borderLen + borderRad, -borderLen + borderRad, borderRad, PI, PI*1.5);
				cairo .line_to (borderLen - borderRad, -borderLen);
				cairo .arc (borderLen - borderRad, -borderLen + borderRad, borderRad, PI*1.5, 0.0);
				cairo .line_to (borderLen, borderLen - borderRad);
				cairo .arc (borderLen - borderRad, borderLen - borderRad, borderRad, 0.0, PI*0.5);
				cairo .line_to (-borderLen + borderRad, borderLen);
				cairo .arc (-borderLen + borderRad, borderLen - borderRad, borderRad, PI*0.5, PI);
				//  My 'grey1' colour:  https://site.second-pair.uk/css/colours-dusk.css
				//  1E (0.1176470588235294) 21 (0.1294117647058824) 2B (0.1686274509803922)
				cairo .set_source_rgba (0.1176470588235294, 0.1294117647058824, 0.1686274509803922, 1.0);
				cairo .fill () .unwrap ();


				//  Perform the draw.
				let anim_type = LogoType ::from_value_or_default (widget .anim_type .get ());
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
