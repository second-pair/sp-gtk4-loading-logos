/*  *--<Preface>--*  //

 -=-  Author Details  -=-
 Blair Edwards
 My own shenanigans.

 -=-  Part Of  -=-
 sp-gtk4-loading-logos.git

 This is mostly here to abstract this mass of code off to a different file.  Depending on what we do later, this may well get merged with 'lib.rs'.

//  *--</Preface>--*  */



//  *--<Doc>--*  */

//!  Documentation!

//  *--</Doc>--*  */



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
use std ::f64 ::consts ::PI;
//  of Which are GTK4
use gtk4 as gtk;
//use gtk ::prelude ::*;
//use gtk ::glib;
//use gtk ::glib ::clone;
use gtk ::cairo ::Context;
use enum_ordinalize ::*;
//  of Which are Local

//  Global Enumerations

/*  Animation Types
1.  Pulsing radius-filling squircle.
2.  Circumference-filling circle CCW.
3.  Circumference-filling circle, CW.
4.  Orbiting N-Ary balls.
5.  Circumference-filling circle, with N-Ary orbiting balls.
6.  N-start circumference-following arcs.
7.  Concentric reverse-direction circumference-following circles.
8.  Concentric reverse-direction circumference-following circles V2.
9.  Concentric reverse-direction circumference-following circles, muType-speed.
10.  Orbiting N-Ary balls, with radius lines.
11.  Orbiting N-Ary balls, with radius-following pulsers.
*/
pub type LtType = i8;
# [repr (i8)]
# [allow (non_camel_case_types)]
# [derive (Default, Clone, Copy, Ordinalize)]
pub enum LogoType
{
	PulseFillCircle,
	CircFillCircleCw,
	CircFillCircleCcw,
	OrbitNBalls,
	# [default]
	CircFillCircle_OrbitNBalls,
	NStartCircArcs,
	ConcentricCircArcsV1,
	ConcentricCircArcsV2,
	ConcentricCircArcsV3,
	OrbitNBalls_RadLines,
	OrbitNBalls_PulseRadLines,
	Pong,
}

//  Global Constants

//  Global Variables

//  Local Constants

//  Local Variables

//  *--</Preparations>--*  //



//  *--<Errors>--*  */

/*pub enum Error
{
	Nope,
}
impl std ::fmt ::Display for Error
{
	fn fmt (&self, fmtr: &mut std ::fmt ::Formatter) -> std ::fmt ::Result
	{
		let errMsg = match self
		{
			Error ::Nope => "Nope error reason.",
		};
		return write! (fmtr, "{}", errMsg);
	}
}
impl std ::fmt ::Debug for Error
{
	fn fmt (&self, fmtr: &mut std ::fmt ::Formatter) -> std ::fmt ::Result
	{
		let errMsg = match self
		{
			Error ::Nope => "Nope error debug reason.",
		};
		return write! (fmtr, "{}", errMsg);
	}
}*/

//  *--</Errors>--*  */



//  *--<Macros>--*  //

/*macro_rules! macroName
{
	($a :expr, $b :expr) =>
	{
		$a + $b
	}
	($a :expr) =>
	{
		$a
	}
}*/

//  *--</Macros>--*  //



//  *--<Traits & Implementations>--*  //

impl LogoType
{
	pub fn to_value (self) -> LtType
	{
		return self .ordinal ();
	}
	pub fn from_value (value: LtType) -> Option <LogoType>
	{
		return LogoType ::from_ordinal (value);
	}
	pub fn from_value_or_default (value: LtType) -> LogoType
	{
		return match LogoType ::from_value (value)
		{
			Some (logo) => logo,
			None => LogoType ::default (),
		};
	}
	pub fn default_value () -> LtType
	{
		return LogoType ::default () .to_value ();
	}
	pub fn max_value () -> LtType
	{
		return (LogoType ::VARIANTS .len () - 1) as LtType;
	}
	//  Cairo Render function.
	//Work out how to break these down into smaller fucntions or something.  Does Rust have private impl functions?
	pub fn draw (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		match (self)
		{
			//#  Parameterise more of these.
			LogoType ::PulseFillCircle => self .draw_PulseFillCircle (cairo, iter, areaScale),
			LogoType ::CircFillCircleCw => self .draw_CircFillCircleCw (cairo, iter, areaScale),
			LogoType ::CircFillCircleCcw => self .draw_CircFillCircleCcw (cairo, iter, areaScale),
			LogoType ::OrbitNBalls => self .draw_OrbitNBalls (cairo, iter, areaScale),
			LogoType ::CircFillCircle_OrbitNBalls => self .draw_CircFillCircle_OrbitNBalls (cairo, iter, areaScale),
			LogoType ::NStartCircArcs => self .draw_NStartCircArcs (cairo, iter, areaScale),
			LogoType ::ConcentricCircArcsV1 => self .draw_ConcentricCircArcsV1 (cairo, iter, areaScale),
			LogoType ::ConcentricCircArcsV2 => self .draw_ConcentricCircArcsV2 (cairo, iter, areaScale),
			LogoType ::ConcentricCircArcsV3 => self .draw_ConcentricCircArcsV3 (cairo, iter, areaScale),
			LogoType ::OrbitNBalls_RadLines => self .draw_OrbitNBalls_RadLines (cairo, iter, areaScale),
			LogoType ::OrbitNBalls_PulseRadLines => self .draw_OrbitNBalls_PulseRadLines (cairo, iter, areaScale),
			LogoType ::Pong => self .draw_Pong (cairo, iter, areaScale),
		}
	}

	//  Given a radius and an angle between 0 - 4.PI, draw the appropriate clockwise-filling circle.
	fn calc_CircFillCircle (self, cairo: &Context, radius: f64, angle: f64, cw: bool)
	{
		//  Draw the circle.
		cairo .move_to (0.0, radius);
		if (cw)
		{
			match (angle <= PI * 2.0)
			{
				true => cairo .arc_negative (0.0, 0.0, radius, PI*0.5, PI*0.5 - angle),
				false => cairo .arc (0.0, 0.0, radius, PI*0.5, PI*2.5 - angle),
			};
		}
		else
		{
			match (angle <= PI * 2.0)
			{
				true => cairo .arc (0.0, 0.0, radius, PI*0.5, PI*0.5 + angle),
				false => cairo .arc_negative (0.0, 0.0, radius, PI*0.5, -PI*1.5 + angle),
			};
		}
	}

	fn draw_PulseFillCircle (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		let radMax = areaScale / 2.0;
		let iterCap = 60.0;
		//  Translate the iterator to a circle radius.
		let radCurr = radMax / iterCap * 2.0 * match (iter % iterCap)
		{
			i if i <= iterCap / 2.0 => i,
			i => iterCap - i,
		};
		//  Draw the circle.
		cairo .move_to (radCurr, 0.0);
		cairo .arc (0.0, 0.0, radCurr, 0.0, PI * 2.0);
	}
	fn draw_CircFillCircleCw (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		let radius = areaScale / 2.0;
		let iterCap = 60.0;
		//  Translate the iterator to a circle radius.
		let angleCurr = PI * 4.0 * (iter % iterCap) / iterCap;
		self .calc_CircFillCircle (cairo, radius, angleCurr, true);
	}
	fn draw_CircFillCircleCcw (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		let radius = areaScale / 2.0;
		let iterCap = 60.0;
		//  Translate the iterator to a circle radius.
		let angleCurr = PI * 4.0 * (iter % iterCap) / iterCap;
		self .calc_CircFillCircle (cairo, radius, angleCurr, false);
	}
	fn draw_OrbitNBalls (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		//  Calculate the overall orbit radius & the radii of the smaller balls.
		let radCircle = areaScale / 10.0;
		let radOrbit = areaScale / 2.0 - radCircle;
		let countCircle = 3;
		let iterCap = 120.0;
		//  Determine the base angular offset.
		let angleBase = PI * 2.0 * (iter % iterCap) / -iterCap;
		//  Step through each ball and draw each evenly interspersed.
		for circle in 0..countCircle
		{
			let angleCurr = angleBase + PI*2.0 * circle as f64 / countCircle as f64;
			cairo .move_to (radCircle + radOrbit * angleCurr .cos (), radOrbit * angleCurr .sin ());
			cairo .arc (radOrbit * angleCurr .cos (), radOrbit * angleCurr .sin (), radCircle, 0.0, PI * 2.0);
		}
	}
	fn draw_CircFillCircle_OrbitNBalls (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		//  Determine the parameters for the outer circle.
		let radOuter = areaScale / 2.0;
		let iterCap = 60.0;
		//  Translate the iterator to a circle radius.
		let angleCurr = PI * 4.0 * (iter % iterCap) / iterCap;
		self .calc_CircFillCircle (cairo, radOuter, angleCurr, true);

		//  Calculate the overall orbit radius & the radii of the smaller balls.
		let radCircle = areaScale / 18.0;
		let radOrbit = radOuter * 0.55 - radCircle;
		let countCircle = 3;
		let iterCap = 142.7;
		//  Determine the base angular offset.
		let angleBase = PI * 2.0 * (iter % iterCap) / -iterCap;
		//  Step through each ball and draw each evenly interspersed.
		for circle in 0..countCircle
		{
			let angleCurr = angleBase + PI*2.0 * circle as f64 / countCircle as f64;
			cairo .move_to (radCircle + radOrbit * angleCurr .cos (), radOrbit * angleCurr .sin ());
			cairo .arc (radOrbit * angleCurr .cos (), radOrbit * angleCurr .sin (), radCircle, 0.0, PI * 2.0);
		}
	}
	fn draw_NStartCircArcs (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		let iterCirc = (iter * 0.07) % (PI * 2.0);
		//  Base parameters, scaling the arc length with the number of starts.
		let radius = areaScale / 2.0;
		let iterCap = 260.0;
		let starts = 5;
		let sliceArc = PI * 2.0 / starts as f64;
		let lengthArc = sliceArc * 0.4;
		let angleStart = PI * 2.0 * (iter % iterCap) / -iterCap;
		for start in 0..starts
		{
			let angleCurr = angleStart + sliceArc * start as f64;
			cairo .move_to (radius * angleCurr .cos (), radius * angleCurr .sin ());
			cairo .arc (0.0, 0.0, radius, angleCurr, angleCurr + lengthArc);
		}
	}
	fn draw_ConcentricCircArcsV1 (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		//  Claculate the per-circle parameters.
		let circleCount = 3;
		let radFirst = areaScale / 2.0;
		let radStep = radFirst / circleCount as f64;
		let iterCap = 60.0;
		//  Translate the iterator into a series of circles.
		let angleCurr = PI * 4.0 * (iter % iterCap) / iterCap;
		for circle in 0..circleCount
		{
			let radCurr = radFirst - radStep * circle as f64;
			let cw = circle % 2 == 0;
			self .calc_CircFillCircle (cairo, radCurr, angleCurr, cw);
		}
	}
	fn draw_ConcentricCircArcsV2 (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		//  Claculate the per-circle parameters.
		let circleCount = 3;
		let radFirst = areaScale / 2.0;
		let radStep = radFirst / circleCount as f64;
		let iterCap = 60.0;
		//  Translate the iterator into a series of circles.
		let angleCurrA = PI * 4.0 * (iter % iterCap) / iterCap;
		let angleCurrB = PI * 4.0 * ((iter + iterCap/2.0) % iterCap) / iterCap;
		for circle in 0..circleCount
		{
			let radCurr = radFirst - radStep * circle as f64;
			match (circle % 2 == 0)
			{
				true => self .calc_CircFillCircle (cairo, radCurr, angleCurrA, true),
				false => self .calc_CircFillCircle (cairo, radCurr, angleCurrB, false),
			}
		}
	}
	fn draw_ConcentricCircArcsV3 (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		//  Claculate the per-circle parameters.
		let circleCount = 3;
		let radFirst = areaScale / 2.0;
		let radStep = radFirst / circleCount as f64;
		//  Translate the iterator into a series of circles.
		for circle in 0..circleCount
		{
			let radCurr = radFirst - radStep * circle as f64;
			let iterCap = 60.0 + 40.0 * circle as f64;
			let angleCurr = PI * 4.0 * (iter % iterCap) / iterCap;
			let cw = circle % 2 == 0;
			self .calc_CircFillCircle (cairo, radCurr, angleCurr, cw);
		}
	}
	fn draw_OrbitNBalls_RadLines (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		let radCircle = 20.0;
		let radOrbit = 200.0;
		let sparkStart = 30.0;
		let sparkGap = 40.0;
		let countCircle = 3;

		for circle in 0..countCircle
		{
			let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
			//  Spark Line
			cairo .move_to (sparkStart * iterStart .cos (), sparkStart * iterStart .sin ());
			cairo .line_to ((radOrbit - radCircle / 2.0 - sparkGap) * iterStart .cos (), (radOrbit - radCircle / 2.0 - sparkGap) * iterStart .sin ());
			//  Circle
			cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
			cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
		}
	}
	fn draw_OrbitNBalls_PulseRadLines (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		let radCircle = 20.0;
		let radOrbit = 200.0;
		let sparkStart = 30.0;
		let sparkGap = 40.0;
		let sparkStop = (radOrbit - radCircle / 2.0 - sparkGap);
		let countCircle = 3;
		let iterSpark = (iter * 8.0) % ((sparkStop - sparkStart) * 2.0);

		for circle in 0..countCircle
		{
			let iterStart = (iter * 0.1 + PI * 2.0 * circle as f64 / countCircle as f64) % (PI * 2.0);
			//  Spark Line
			match (iterSpark <= sparkStop - sparkStart)
			{
				true =>
				{
					cairo .move_to (sparkStart * iterStart .cos (), sparkStart * iterStart .sin ());
					cairo .line_to ((sparkStart + iterSpark) * iterStart .cos (), (sparkStart + iterSpark) * iterStart .sin ());
				},
				false =>
				{
					let iterSpark = iterSpark % (sparkStop - sparkStart);
					cairo .move_to ((sparkStart + iterSpark) * iterStart .cos (), (sparkStart + iterSpark) * iterStart .sin ());
					cairo .line_to (sparkStop * iterStart .cos (), sparkStop * iterStart .sin ());
				},
			};
			//  Circle
			cairo .move_to (radCircle + radOrbit * iterStart .cos (), radOrbit * iterStart .sin ());
			cairo .arc (radOrbit * iterStart .cos (), radOrbit * iterStart .sin (), radCircle, 0.0, PI * 2.0);
		}
	}

	fn draw_Pong (self, cairo: &Context, iter: f64, areaScale: f64)
	{ todo! () }
}

//  *--</Traits & Implementations>--*  //



//  *--<Main Code>--*  //

/*fn subLocal () -> u8
{
	return 0;
}*/

//  *--</Main Code>--*  //



//  *--<Callbacks>--*  //

//  *--</Callbacks>--*  //
