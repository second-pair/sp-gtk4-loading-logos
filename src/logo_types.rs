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
	OrbitNBalls_SparkLines,
	OrbitNBalls_PulseSparkLines,
	Pong,
}

struct PongPoint
{
	near: f64,
	off: f64,
	cycles: f64,
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
			LogoType ::OrbitNBalls_SparkLines => self .draw_OrbitNBalls_SparkLines (cairo, iter, areaScale),
			LogoType ::OrbitNBalls_PulseSparkLines => self .draw_OrbitNBalls_PulseSparkLines (cairo, iter, areaScale),
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
	fn calc_OrbitNBalls (self, cairo: &Context, radOrbit: f64, radBall: f64, count: usize, angle: f64, cw: bool)
	{
		//  Step through each ball and draw each evenly interspersed.
		for ball in 0..count
		{
			let angleCurr = match cw
			{
				true => angle + PI*2.0 * ball as f64 / count as f64,
				false => PI * 2.0 - (angle + PI*2.0 * ball as f64 / count as f64),
			};
			cairo .move_to (radBall + radOrbit * angleCurr .cos (), radOrbit * angleCurr .sin ());
			cairo .arc (radOrbit * angleCurr .cos (), radOrbit * angleCurr .sin (), radBall, 0.0, PI*2.0);
		}
	}
	fn calc_SparkLines (self, cairo: &Context, radS: f64, radE: f64, count: usize, angle: f64, cw: bool)
	{
		for spark in 0..count
		{
			let angleCurr = match cw
			{
				true => angle + PI*2.0 * spark as f64 / count as f64,
				false => PI * 2.0 - (angle + PI*2.0 * spark as f64 / count as f64),
			};
			cairo .move_to (radS * angleCurr .cos (), radS * angleCurr .sin ());
			cairo .line_to (radE * angleCurr .cos (), radE * angleCurr .sin ());
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
		let radBall = areaScale / 10.0;
		let radOrbit = areaScale / 2.0 - radBall;
		let countBall = 3;
		let iterCap = 120.0;
		//  Determine the base angular offset.
		let angleBase = PI * 2.0 * (iter % iterCap) / -iterCap;
		self .calc_OrbitNBalls (cairo, radOrbit, radBall, countBall, angleBase, true);
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
		let radBall = areaScale / 18.0;
		let radOrbit = radOuter * 0.55 - radBall;
		let countBall = 3;
		let iterCap = 142.7;
		//  Determine the base angular offset.
		let angleBase = PI * 2.0 * (iter % iterCap) / -iterCap;
		self .calc_OrbitNBalls (cairo, radOrbit, radBall, countBall, angleBase, true);
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
	fn draw_OrbitNBalls_SparkLines (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		//  Calculate the overall orbit radius & the radii of the smaller balls.
		let radBall = areaScale / 18.0;
		let radOrbit = areaScale / 2.0 - radBall;
		let sepFactorS = 1.6;
		let sepFactorE = 2.8;
		let count = 3;
		let cw = false;
		let iterCap = 60.0;
		//  Determine the base angular offset.
		let angleBase = PI * 2.0 * (iter % iterCap) / -iterCap;
		self .calc_OrbitNBalls (cairo, radOrbit, radBall, count, angleBase, cw);

		//  Calculate positions for the sparklines.
		let radStart = radBall * sepFactorS;
		let radEnd = radOrbit - radBall * sepFactorE;
		self .calc_SparkLines (cairo, radStart, radEnd, count, angleBase, cw);
	}
	fn draw_OrbitNBalls_PulseSparkLines (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		//  Calculate the overall orbit radius & the radii of the smaller balls.
		let radBall = areaScale / 18.0;
		let radOrbit = areaScale / 2.0 - radBall;
		let sepFactorS = 1.6;
		let sepFactorE = 2.8;
		let count = 3;
		let cw = false;
		let iterCapBalls = 60.0;
		//  Determine the base angular offset.
		let angleBase = PI * 2.0 * (iter % iterCapBalls) / -iterCapBalls;
		self .calc_OrbitNBalls (cairo, radOrbit, radBall, count, angleBase, cw);

		//  Calculate the extents of the sparklines.
		let radStart = radBall * sepFactorS;
		let radLen = radOrbit - radBall * sepFactorE - radStart;
		//  Calculate an iterator-length value.
		let iterCapSparks = 40.0;
		let lenIter = radLen * (iter % iterCapSparks) / iterCapSparks * 2.0;
		//  Determine the actual start & end based off this.
		match (lenIter)
		{
			x if x > radLen =>
			{
				let radCurrS = radStart + (lenIter - radLen);
				let radCurrE = radStart + radLen;
				self .calc_SparkLines (cairo, radCurrS, radCurrE, count, angleBase, cw);
			},
			x =>
			{
				let radCurrS = radStart;
				let radCurrE = radStart + lenIter;
				self .calc_SparkLines (cairo, radCurrS, radCurrE, count, angleBase, cw);
			},
		}
	}

	/*  Pong
	This one is going to need a bit of planning...
	Let's create a loop that sees the ball bounce to a few positions on each side.
	We can sequence the ball & paddles to move to these positions linearly in turn, over some number of iteration cycles.
	It would also be neat if the offside paddle didn't just mirror the earside paddle - maybe it returns to the centre each time?
	*/
	fn draw_Pong (self, cairo: &Context, iter: f64, areaScale: f64)
	{
		//  To start with, let's define a series of points, draw a left-hand paddle and have it cycle between these.
		let mut points: Vec <PongPoint> = vec! (
			PongPoint {near: 200.0, off: 0.0, cycles: 60.0},
			PongPoint {near: 170.0, off: 0.0, cycles: 70.0},
			PongPoint {near: 350.0, off: 0.0, cycles: 120.0},
		);
		//cycTotal = 250.0;  //  There will be some neat way of creating an iterator out of the struct-vector and collecting them all in a one-er.

		//  Let's get going with just one point.  We'll calculate where the Nearside paddle should be and draw a rectangular paddle there.
		let padHeight = areaScale * 0.1;
		let ballRad = areaScale * 0.01;
		let pointLast = PongPoint {near: -0.3, off: -0.2, cycles: 60.0};
		let pointCurr = PongPoint {near: 0.4, off: 0.1, cycles: 60.0};

		//  Near Paddle - moving from Last Off to Current Near.
		let posNear = (pointLast .off + (pointCurr .near - pointLast .off) * (iter % pointCurr .cycles) / pointCurr .cycles) * areaScale;
		cairo .move_to (-areaScale/2.0, posNear - padHeight/2.0);
		cairo .line_to (-areaScale/2.0, posNear + padHeight/2.0);

		//  Far Paddle - moving from Last Near to Current Off.
		let posOff = (pointLast .near + (pointCurr .off - pointLast .near) * (iter % pointCurr .cycles) / pointCurr .cycles) * areaScale;
		cairo .move_to (areaScale/2.0, posOff - padHeight/2.0);
		cairo .line_to (areaScale/2.0, posOff + padHeight/2.0);

		//  Render that line.
		cairo .stroke () .unwrap ();
		//  Ball - moving from Last Near to Current Near.
		let posBallY = (pointLast .near + (pointCurr .near - pointLast .near) * (iter % pointCurr .cycles) / pointCurr .cycles) * areaScale;
		let posBallX = areaScale/2.0 - areaScale * (iter % pointCurr .cycles) / pointCurr .cycles;
		cairo .move_to (posBallX+ballRad, posBallY);
		cairo .arc (posBallX, posBallY, ballRad, 0.0, PI*2.0);
		cairo .fill () .unwrap ();
	}
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
