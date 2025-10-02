//! Transition Curves
//! 
//! This module provides various transition curves for smooth and natural
//! music transitions including linear, exponential, logarithmic, and custom curves.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Transition curve types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransitionCurve {
    /// Linear curve (constant rate)
    Linear,
    /// Exponential curve (accelerating)
    Exponential {
        /// Exponential factor
        factor: f32,
    },
    /// Logarithmic curve (decelerating)
    Logarithmic {
        /// Logarithmic factor
        factor: f32,
    },
    /// Sine curve (smooth S-curve)
    Sine,
    /// Cosine curve (smooth S-curve)
    Cosine,
    /// Smooth step curve (smooth S-curve)
    SmoothStep,
    /// Smoother step curve (smoother S-curve)
    SmootherStep,
    /// Bezier curve (custom control points)
    Bezier {
        /// Control point 1 X
        cp1_x: f32,
        /// Control point 1 Y
        cp1_y: f32,
        /// Control point 2 X
        cp2_x: f32,
        /// Control point 2 Y
        cp2_y: f32,
    },
    /// Hermite curve (smooth interpolation)
    Hermite {
        /// Tangent 1
        tangent1: f32,
        /// Tangent 2
        tangent2: f32,
    },
    /// Catmull-Rom curve (smooth spline)
    CatmullRom {
        /// Control point 1
        cp1: f32,
        /// Control point 2
        cp2: f32,
    },
    /// B-spline curve (smooth spline)
    BSpline {
        /// Control point 1
        cp1: f32,
        /// Control point 2
        cp2: f32,
        /// Control point 3
        cp3: f32,
    },
    /// Ease in curve (slow start)
    EaseIn {
        /// Ease in factor
        factor: f32,
    },
    /// Ease out curve (slow end)
    EaseOut {
        /// Ease out factor
        factor: f32,
    },
    /// Ease in-out curve (slow start and end)
    EaseInOut {
        /// Ease in-out factor
        factor: f32,
    },
    /// Ease in cubic curve
    EaseInCubic,
    /// Ease out cubic curve
    EaseOutCubic,
    /// Ease in-out cubic curve
    EaseInOutCubic,
    /// Ease in quartic curve
    EaseInQuartic,
    /// Ease out quartic curve
    EaseOutQuartic,
    /// Ease in-out quartic curve
    EaseInOutQuartic,
    /// Ease in quintic curve
    EaseInQuintic,
    /// Ease out quintic curve
    EaseOutQuintic,
    /// Ease in-out quintic curve
    EaseInOutQuintic,
    /// Ease in sine curve
    EaseInSine,
    /// Ease out sine curve
    EaseOutSine,
    /// Ease in-out sine curve
    EaseInOutSine,
    /// Ease in exponential curve
    EaseInExpo,
    /// Ease out exponential curve
    EaseOutExpo,
    /// Ease in-out exponential curve
    EaseInOutExpo,
    /// Ease in circular curve
    EaseInCirc,
    /// Ease out circular curve
    EaseOutCirc,
    /// Ease in-out circular curve
    EaseInOutCirc,
    /// Ease in back curve (overshoot)
    EaseInBack {
        /// Back overshoot factor
        overshoot: f32,
    },
    /// Ease out back curve (overshoot)
    EaseOutBack {
        /// Back overshoot factor
        overshoot: f32,
    },
    /// Ease in-out back curve (overshoot)
    EaseInOutBack {
        /// Back overshoot factor
        overshoot: f32,
    },
    /// Ease in elastic curve (bounce)
    EaseInElastic {
        /// Elastic amplitude
        amplitude: f32,
        /// Elastic period
        period: f32,
    },
    /// Ease out elastic curve (bounce)
    EaseOutElastic {
        /// Elastic amplitude
        amplitude: f32,
        /// Elastic period
        period: f32,
    },
    /// Ease in-out elastic curve (bounce)
    EaseInOutElastic {
        /// Elastic amplitude
        amplitude: f32,
        /// Elastic period
        period: f32,
    },
    /// Ease in bounce curve
    EaseInBounce {
        /// Bounce intensity
        intensity: f32,
    },
    /// Ease out bounce curve
    EaseOutBounce {
        /// Bounce intensity
        intensity: f32,
    },
    /// Ease in-out bounce curve
    EaseInOutBounce {
        /// Bounce intensity
        intensity: f32,
    },
    /// Step curve (discrete steps)
    Step {
        /// Number of steps
        steps: u32,
    },
    /// Sawtooth curve (linear up, instant down)
    Sawtooth,
    /// Triangle curve (linear up and down)
    Triangle,
    /// Square curve (instant up and down)
    Square,
    /// Custom curve (user-defined function)
    Custom {
        /// Custom curve name
        name: String,
        /// Custom curve parameters
        parameters: HashMap<String, f32>,
    },
}

/// Transition curve utilities
impl TransitionCurve {
    /// Evaluate curve at given progress (0.0 to 1.0)
    pub fn evaluate(&self, progress: f32) -> f32 {
        let t = progress.max(0.0).min(1.0);
        
        match self {
            TransitionCurve::Linear => t,
            
            TransitionCurve::Exponential { factor } => {
                if *factor > 0.0 {
                    (t.powf(*factor) - 1.0) / (factor - 1.0)
                } else {
                    t
                }
            },
            
            TransitionCurve::Logarithmic { factor } => {
                if *factor > 0.0 && *factor != 1.0 {
                    (factor.powf(t) - 1.0) / (factor - 1.0)
                } else {
                    t
                }
            },
            
            TransitionCurve::Sine => {
                (1.0 - (t * std::f32::consts::PI).cos()) / 2.0
            },
            
            TransitionCurve::Cosine => {
                (t * std::f32::consts::PI).sin() / 2.0 + 0.5
            },
            
            TransitionCurve::SmoothStep => {
                t * t * (3.0 - 2.0 * t)
            },
            
            TransitionCurve::SmootherStep => {
                t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
            },
            
            TransitionCurve::Bezier { cp1_x, cp1_y, cp2_x, cp2_y } => {
                self.cubic_bezier(t, 0.0, *cp1_x, *cp2_x, 1.0)
            },
            
            TransitionCurve::Hermite { tangent1, tangent2 } => {
                self.hermite_interpolation(t, 0.0, 1.0, *tangent1, *tangent2)
            },
            
            TransitionCurve::CatmullRom { cp1, cp2 } => {
                self.catmull_rom_interpolation(t, 0.0, *cp1, *cp2, 1.0)
            },
            
            TransitionCurve::BSpline { cp1, cp2, cp3 } => {
                self.b_spline_interpolation(t, 0.0, *cp1, *cp2, 1.0)
            },
            
            TransitionCurve::EaseIn { factor } => {
                t.powf(*factor)
            },
            
            TransitionCurve::EaseOut { factor } => {
                1.0 - (1.0 - t).powf(*factor)
            },
            
            TransitionCurve::EaseInOut { factor } => {
                if t < 0.5 {
                    2.0 * t.powf(*factor) / 2.0
                } else {
                    1.0 - 2.0 * (1.0 - t).powf(*factor) / 2.0
                }
            },
            
            TransitionCurve::EaseInCubic => t * t * t,
            TransitionCurve::EaseOutCubic => 1.0 - (1.0 - t).powi(3),
            TransitionCurve::EaseInOutCubic => {
                if t < 0.5 {
                    4.0 * t * t * t
                } else {
                    1.0 - 4.0 * (1.0 - t).powi(3)
                }
            },
            
            TransitionCurve::EaseInQuartic => t * t * t * t,
            TransitionCurve::EaseOutQuartic => 1.0 - (1.0 - t).powi(4),
            TransitionCurve::EaseInOutQuartic => {
                if t < 0.5 {
                    8.0 * t * t * t * t
                } else {
                    1.0 - 8.0 * (1.0 - t).powi(4)
                }
            },
            
            TransitionCurve::EaseInQuintic => t * t * t * t * t,
            TransitionCurve::EaseOutQuintic => 1.0 - (1.0 - t).powi(5),
            TransitionCurve::EaseInOutQuintic => {
                if t < 0.5 {
                    16.0 * t * t * t * t * t
                } else {
                    1.0 - 16.0 * (1.0 - t).powi(5)
                }
            },
            
            TransitionCurve::EaseInSine => 1.0 - (t * std::f32::consts::PI / 2.0).cos(),
            TransitionCurve::EaseOutSine => (t * std::f32::consts::PI / 2.0).sin(),
            TransitionCurve::EaseInOutSine => {
                -(t * std::f32::consts::PI).cos() / 2.0 + 0.5
            },
            
            TransitionCurve::EaseInExpo => {
                if t == 0.0 { 0.0 } else { 2.0_f32.powf(10.0 * (t - 1.0)) }
            },
            TransitionCurve::EaseOutExpo => {
                if t == 1.0 { 1.0 } else { 1.0 - 2.0_f32.powf(-10.0 * t) }
            },
            TransitionCurve::EaseInOutExpo => {
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else if t < 0.5 { 2.0_f32.powf(20.0 * t - 10.0) / 2.0 }
                else { (2.0 - 2.0_f32.powf(-20.0 * t + 10.0)) / 2.0 }
            },
            
            TransitionCurve::EaseInCirc => 1.0 - (1.0 - t * t).sqrt(),
            TransitionCurve::EaseOutCirc => (1.0 - (t - 1.0) * (t - 1.0)).sqrt(),
            TransitionCurve::EaseInOutCirc => {
                if t < 0.5 {
                    (1.0 - (1.0 - 4.0 * t * t).sqrt()) / 2.0
                } else {
                    (1.0 + (1.0 - 4.0 * (t - 1.0) * (t - 1.0)).sqrt()) / 2.0
                }
            },
            
            TransitionCurve::EaseInBack { overshoot } => {
                let c1 = 1.70158 + overshoot;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            },
            TransitionCurve::EaseOutBack { overshoot } => {
                let c1 = 1.70158 + overshoot;
                let c3 = c1 + 1.0;
                1.0 + c3 * (t - 1.0).powi(3) + c1 * (t - 1.0).powi(2)
            },
            TransitionCurve::EaseInOutBack { overshoot } => {
                let c1 = 1.70158 + overshoot;
                let c2 = c1 * 1.525;
                if t < 0.5 {
                    (2.0 * t).powi(2) * ((c2 + 1.0) * 2.0 * t - c2) / 2.0
                } else {
                    ((2.0 * t - 2.0).powi(2) * ((c2 + 1.0) * (2.0 * t - 2.0) + c2) + 2.0) / 2.0
                }
            },
            
            TransitionCurve::EaseInElastic { amplitude, period } => {
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else {
                    -amplitude * 2.0_f32.powf(10.0 * (t - 1.0)) * 
                    (t - 1.0 - period / 4.0 * (2.0 * std::f32::consts::PI / period).sin()).sin()
                }
            },
            TransitionCurve::EaseOutElastic { amplitude, period } => {
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else {
                    amplitude * 2.0_f32.powf(-10.0 * t) * 
                    (t - period / 4.0 * (2.0 * std::f32::consts::PI / period).sin()).sin() + 1.0
                }
            },
            TransitionCurve::EaseInOutElastic { amplitude, period } => {
                if t == 0.0 { 0.0 }
                else if t == 1.0 { 1.0 }
                else if t < 0.5 {
                    -(amplitude * 2.0_f32.powf(20.0 * t - 10.0) * 
                      (20.0 * t - 10.0 - period / 4.0 * (2.0 * std::f32::consts::PI / period).sin()).sin()) / 2.0
                } else {
                    amplitude * 2.0_f32.powf(-20.0 * t + 10.0) * 
                    (20.0 * t - 10.0 - period / 4.0 * (2.0 * std::f32::consts::PI / period).sin()).sin() / 2.0 + 1.0
                }
            },
            
            TransitionCurve::EaseInBounce { intensity } => {
                1.0 - self.ease_out_bounce(1.0 - t, *intensity)
            },
            TransitionCurve::EaseOutBounce { intensity } => {
                self.ease_out_bounce(t, *intensity)
            },
            TransitionCurve::EaseInOutBounce { intensity } => {
                if t < 0.5 {
                    (1.0 - self.ease_out_bounce(1.0 - 2.0 * t, *intensity)) / 2.0
                } else {
                    (1.0 + self.ease_out_bounce(2.0 * t - 1.0, *intensity)) / 2.0
                }
            },
            
            TransitionCurve::Step { steps } => {
                if *steps == 0 { t } else { (t * *steps as f32).floor() / *steps as f32 }
            },
            
            TransitionCurve::Sawtooth => t,
            TransitionCurve::Triangle => {
                if t < 0.5 { 2.0 * t } else { 2.0 - 2.0 * t }
            },
            TransitionCurve::Square => {
                if t < 0.5 { 0.0 } else { 1.0 }
            },
            
            TransitionCurve::Custom { .. } => {
                // Custom curve evaluation would be implemented here
                // For now, return linear
                t
            },
        }
    }

    /// Cubic Bezier interpolation
    fn cubic_bezier(&self, t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
        let u = 1.0 - t;
        let tt = t * t;
        let uu = u * u;
        let uuu = uu * u;
        let ttt = tt * t;
        
        uuu * p0 + 3.0 * uu * t * p1 + 3.0 * u * tt * p2 + ttt * p3
    }

    /// Hermite interpolation
    fn hermite_interpolation(&self, t: f32, p0: f32, p1: f32, m0: f32, m1: f32) -> f32 {
        let tt = t * t;
        let ttt = tt * t;
        
        (2.0 * ttt - 3.0 * tt + 1.0) * p0 +
        (ttt - 2.0 * tt + t) * m0 +
        (-2.0 * ttt + 3.0 * tt) * p1 +
        (ttt - tt) * m1
    }

    /// Catmull-Rom interpolation
    fn catmull_rom_interpolation(&self, t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
        let tt = t * t;
        let ttt = tt * t;
        
        0.5 * (
            (2.0 * p1) +
            (-p0 + p2) * t +
            (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * tt +
            (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * ttt
        )
    }

    /// B-spline interpolation
    fn b_spline_interpolation(&self, t: f32, p0: f32, p1: f32, p2: f32, p3: f32) -> f32 {
        let tt = t * t;
        let ttt = tt * t;
        
        (p0 + 4.0 * p1 + p2) / 6.0 +
        (p1 - p0) * t / 2.0 +
        (p0 - 2.0 * p1 + p2) * tt / 2.0 +
        (p3 - p0 + 3.0 * (p1 - p2)) * ttt / 6.0
    }

    /// Ease out bounce helper
    fn ease_out_bounce(&self, t: f32, intensity: f32) -> f32 {
        let n1 = 7.5625;
        let d1 = 2.75;
        
        if t < 1.0 / d1 {
            n1 * t * t
        } else if t < 2.0 / d1 {
            let t = t - 1.5 / d1;
            n1 * t * t + 0.75
        } else if t < 2.5 / d1 {
            let t = t - 2.25 / d1;
            n1 * t * t + 0.9375
        } else {
            let t = t - 2.625 / d1;
            n1 * t * t + 0.984375
        }
    }

    /// Get curve name
    pub fn get_name(&self) -> &'static str {
        match self {
            TransitionCurve::Linear => "Linear",
            TransitionCurve::Exponential { .. } => "Exponential",
            TransitionCurve::Logarithmic { .. } => "Logarithmic",
            TransitionCurve::Sine => "Sine",
            TransitionCurve::Cosine => "Cosine",
            TransitionCurve::SmoothStep => "SmoothStep",
            TransitionCurve::SmootherStep => "SmootherStep",
            TransitionCurve::Bezier { .. } => "Bezier",
            TransitionCurve::Hermite { .. } => "Hermite",
            TransitionCurve::CatmullRom { .. } => "CatmullRom",
            TransitionCurve::BSpline { .. } => "BSpline",
            TransitionCurve::EaseIn { .. } => "EaseIn",
            TransitionCurve::EaseOut { .. } => "EaseOut",
            TransitionCurve::EaseInOut { .. } => "EaseInOut",
            TransitionCurve::EaseInCubic => "EaseInCubic",
            TransitionCurve::EaseOutCubic => "EaseOutCubic",
            TransitionCurve::EaseInOutCubic => "EaseInOutCubic",
            TransitionCurve::EaseInQuartic => "EaseInQuartic",
            TransitionCurve::EaseOutQuartic => "EaseOutQuartic",
            TransitionCurve::EaseInOutQuartic => "EaseInOutQuartic",
            TransitionCurve::EaseInQuintic => "EaseInQuintic",
            TransitionCurve::EaseOutQuintic => "EaseOutQuintic",
            TransitionCurve::EaseInOutQuintic => "EaseInOutQuintic",
            TransitionCurve::EaseInSine => "EaseInSine",
            TransitionCurve::EaseOutSine => "EaseOutSine",
            TransitionCurve::EaseInOutSine => "EaseInOutSine",
            TransitionCurve::EaseInExpo => "EaseInExpo",
            TransitionCurve::EaseOutExpo => "EaseOutExpo",
            TransitionCurve::EaseInOutExpo => "EaseInOutExpo",
            TransitionCurve::EaseInCirc => "EaseInCirc",
            TransitionCurve::EaseOutCirc => "EaseOutCirc",
            TransitionCurve::EaseInOutCirc => "EaseInOutCirc",
            TransitionCurve::EaseInBack { .. } => "EaseInBack",
            TransitionCurve::EaseOutBack { .. } => "EaseOutBack",
            TransitionCurve::EaseInOutBack { .. } => "EaseInOutBack",
            TransitionCurve::EaseInElastic { .. } => "EaseInElastic",
            TransitionCurve::EaseOutElastic { .. } => "EaseOutElastic",
            TransitionCurve::EaseInOutElastic { .. } => "EaseInOutElastic",
            TransitionCurve::EaseInBounce { .. } => "EaseInBounce",
            TransitionCurve::EaseOutBounce { .. } => "EaseOutBounce",
            TransitionCurve::EaseInOutBounce { .. } => "EaseInOutBounce",
            TransitionCurve::Step { .. } => "Step",
            TransitionCurve::Sawtooth => "Sawtooth",
            TransitionCurve::Triangle => "Triangle",
            TransitionCurve::Square => "Square",
            TransitionCurve::Custom { name, .. } => name,
        }
    }

    /// Check if curve is smooth
    pub fn is_smooth(&self) -> bool {
        !matches!(
            self,
            TransitionCurve::Step { .. } | 
            TransitionCurve::Sawtooth | 
            TransitionCurve::Square
        )
    }

    /// Check if curve is continuous
    pub fn is_continuous(&self) -> bool {
        !matches!(
            self,
            TransitionCurve::Step { .. } | 
            TransitionCurve::Square
        )
    }
}

impl Default for TransitionCurve {
    fn default() -> Self {
        TransitionCurve::Linear
    }
}
