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
#! [allow (dead_code)]
#! [allow (unused_imports)]

//  Imports
//use std ::io ::{stdin, stdout};
//use std ::io ::Result;
//use std ::time;
//use std ::fs ::File;
//use std ::path ::Path;
//use std ::io ::{Read, BufReader, BufRead};
//use std ::io ::{Write, BufWriter};
use std ::f64 ::consts ::PI;
use std ::sync ::atomic ::{AtomicUsize, Ordering};
//use std ::rc ::Rc;
//use std ::cell ::RefCell;
//  of Which are GTK4
use gtk4 as gtk;
use gtk ::prelude ::*;
use gtk ::glib;
use gtk ::glib ::clone;
//  of Which are Local
mod loading_logos;
use loading_logos ::LoadingLogo;

//  Global Enumerations

//  Global Constants
const TIME_ANIM: u16 = 50;
const DRAW_TARGET_LEN: f64 = 1000.0;
const DRAW_LINE_WIDTH_BASE: f64 = 10.0;

//  Global Variables

//  Local Constants

//  Local Variables

//  Structures

//  *--</Preparations>--*  //



//  *--<Macros>--*  //

//  *--</Macros>--*  //



//  *--<Traits & Implementations>--*  //

//  *--</Traits & Implementations>--*  //



//  *--<Main Code>--*  //

# [no_mangle]
pub unsafe extern "C" fn sp_gtk4_loading_logos_create () -> gtk ::Widget
{
	return priv_create ();
}

fn priv_create () -> gtk ::Widget
{
	//  Attach the Cairo canvas.
	let cairo_loading = gtk ::DrawingArea ::builder ()
		.content_width (400)
		.content_height (400)
		.build ();
	cairo_loading .set_draw_func (cairo_loading_render);
	//  Add a timeout to update the logo's positions.
	//  Local, so we don't mess with GTK's main-thread requirements.
	gtk ::glib ::timeout_add_local
	(
		core ::time ::Duration ::from_millis (TIME_ANIM as u64),
		clone! (@strong cairo_loading => move ||
		{
			cairo_loading .queue_draw ();
			return true .into ();
		}
	));
	return cairo_loading .into ();
}

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

fn cairo_loading_render (area: &gtk ::DrawingArea, cairo: &gtk ::cairo ::Context, width: i32, height: i32)
{
	//  'static' iteration counter.
	//#  Handle overflow.
	static ITER: AtomicUsize = AtomicUsize ::new (0);
	let iter = ITER .fetch_add (1, Ordering ::Relaxed) as f64;

	//  Draw a box outline to help suss out the widget's area.
	cairo .rectangle (0.0, 0.0, width as f64, height as f64);

	//  Scale factor - calculated from 'width' and 'height'.
	let areaScale = core ::cmp ::min (width, height) as f64 / DRAW_TARGET_LEN;

	//  Move the origin to the middle and flip the Y-axis.
	let matrix = gtk ::cairo ::Matrix ::new (1.0, 0.0, 0.0, -1.0, width as f64 / 2.0, height as f64 / 2.0);
	cairo .transform (matrix);

	//  Perform the draw.
	let ANIM_TYPE: LoadingLogo = LoadingLogo ::OrbitNBalls;
	ANIM_TYPE .draw (cairo, iter, areaScale);

	//  Render that line.
	cairo .set_line_width (DRAW_LINE_WIDTH_BASE * areaScale);
	cairo .set_line_cap (gtk ::cairo ::LineCap ::Round);
	cairo .set_line_join (gtk ::cairo ::LineJoin ::Round);
	cairo .set_source_rgba (1.0, 1.0, 1.0, 1.0);
	cairo .stroke () .unwrap ();
}

//  *--</Callbacks>--*  //
