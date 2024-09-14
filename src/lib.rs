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
mod loading_logos;
use loading_logos ::LoadingLogo;

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

glib ::wrapper!
{
	pub struct Logo1 (ObjectSubclass <loading_logos_impl ::Logo1>)
	@extends gtk ::DrawingArea, gtk ::Widget,
	@implements gtk ::Accessible, gtk ::Actionable, gtk ::Buildable, gtk::ConstraintTarget;
}

impl Logo1
{
	pub fn create () -> Self
	{
		let da_logo: Logo1 = Object ::builder () .build ();
		da_logo .set_hexpand (true);
		da_logo .set_vexpand (true);
		da_logo .set_draw_func (priv_logo_render);

		//  Add a timeout to update the logo's positions.
		//  Local, so we don't mess with GTK's main-thread requirements.
		gtk ::glib ::timeout_add_local
		(
			//core ::time ::Duration ::from_millis (TIME_ANIM as u64),
			core ::time ::Duration ::from_millis (1000 as u64),
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

# [no_mangle]
pub unsafe extern "C" fn sp_gtk4_loading_logos_create () -> gtk ::Widget
{
	gtk ::init () .unwrap ();

	//return priv_create ();
	return Logo1 ::create () .into ();
}

fn priv_create () -> gtk ::Widget
{
	//  Create the DrawingArea.
	let cairo_logo = gtk ::DrawingArea ::builder ()
		.hexpand (true)
		.vexpand (true)
		.build ();

	cairo_logo .set_draw_func (priv_logo_render);

	//  Create the animation enum.
	let anim_type: LoadingLogo = LoadingLogo ::OrbitNBalls;
	//g_object_set_data (&mut cairo_logo, "anim_type", anim_type .into ());

	//  Add a timeout to update the logo's positions.
	//  Local, so we don't mess with GTK's main-thread requirements.
	gtk ::glib ::timeout_add_local
	(
		core ::time ::Duration ::from_millis (TIME_ANIM as u64),
		clone! (#[strong] cairo_logo, move ||
		{
			cairo_logo .queue_draw ();
			return ControlFlow ::Continue;
		}
	));

	return cairo_logo .into ();
}

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

fn priv_logo_render (area: &gtk ::DrawingArea, cairo: &gtk ::cairo ::Context, width: i32, height: i32)
{
	let anim_type: LoadingLogo = LoadingLogo ::OrbitNBalls;
	anim_type .render (area, cairo, width, height);
}

//  *--</Callbacks>--*  //


mod loading_logos_impl
{
	use gtk4 as gtk;
	use gtk::glib;
	use gtk::subclass::prelude::*;

	# [derive (Default)]
	pub struct Logo1
	{
		//anim_type: LoadingLogo,
		iter: f64,
	}

	# [glib ::object_subclass]
	impl ObjectSubclass for Logo1
	{
		const NAME:  &'static str = "Logo1";
		type Type = super ::Logo1;
		type ParentType = gtk ::DrawingArea;
	}
	//  Trait shared by all GObjects
	impl ObjectImpl for Logo1 {}
	//  Trait shared by all widgets
	impl WidgetImpl for Logo1 {}
	//  Trait shared by all drawing areas
	impl DrawingAreaImpl for Logo1 {}
}
