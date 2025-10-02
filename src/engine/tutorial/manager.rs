//! Tutorial manager
//! 
//! This module provides the main tutorial management system.

use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use super::{
    TutorialStep, TutorialProgress, TutorialConfig, TutorialSettings, 
    TutorialStats, TutorialEvent, TutorialError, TutorialResult,
    TutorialUI, TutorialOverlay, TutorialHighlight
};
use super::step::TutorialContext;

/// Tutorial implementation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tutorial {
    /// Tutorial ID
    pub id: String,
    /// Tutorial name
    pub name: String,
    /// Tutorial description
    pub description: String,
    /// Tutorial steps
    pub steps: Vec<TutorialStep>,
    /// Prerequisites
    pub prerequisites: Vec<String>,
    /// Rewards
    pub rewards: Vec<super::TutorialReward>,
}

impl Tutorial {
    /// Create a new tutorial
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description,
            steps: Vec::new(),
            prerequisites: Vec::new(),
            rewards: Vec::new(),
        }
    }

    /// Add step
    pub fn add_step(&mut self, step: TutorialStep) {
        self.steps.push(step);
    }

    /// Add prerequisite
    pub fn add_prerequisite(&mut self, prerequisite: String) {
        self.prerequisites.push(prerequisite);
    }

    /// Add reward
    pub fn add_reward(&mut self, reward: super::TutorialReward) {
        self.rewards.push(reward);
    }

    /// Get step count
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }

    /// Get step by index
    pub fn get_step(&self, index: usize) -> Option<&TutorialStep> {
        self.steps.get(index)
    }
}

/// Main tutorial manager
pub struct TutorialManager {
    /// Available tutorials
    pub tutorials: HashMap<String, Tutorial>,
    /// Tutorial progress
    pub progress: HashMap<String, TutorialProgress>,
    /// Active tutorial
    pub active_tutorial: Option<String>,
    /// Current step
    pub current_step: Option<usize>,
    /// Completed tutorials
    pub completed_tutorials: HashSet<String>,
    /// Configuration
    pub config: TutorialConfig,
    /// Settings
    pub settings: TutorialSettings,
    /// Statistics
    pub stats: TutorialStats,
    /// UI system
    pub ui: TutorialUI,
    /// Event handlers
    pub event_handlers: HashMap<TutorialEvent, Vec<Box<dyn Fn(&TutorialEvent) + Send + Sync>>>,
}

impl TutorialManager {
    /// Create a new tutorial manager
    pub fn new() -> Self {
        Self {
            tutorials: HashMap::new(),
            progress: HashMap::new(),
            active_tutorial: None,
            current_step: None,
            completed_tutorials: HashSet::new(),
            config: TutorialConfig::default(),
            settings: TutorialSettings::default(),
            stats: TutorialStats::default(),
            ui: TutorialUI::new(),
            event_handlers: HashMap::new(),
        }
    }

    /// Add a tutorial
    pub fn add_tutorial(&mut self, tutorial: Tutorial) -> TutorialResult<()> {
        if self.tutorials.contains_key(&tutorial.id) {
            return Err(TutorialError::ConfigError(format!("Tutorial '{}' already exists", tutorial.id)));
        }

        self.tutorials.insert(tutorial.id.clone(), tutorial);
        self.progress.insert(tutorial.id.clone(), TutorialProgress::new(tutorial.id));
        Ok(())
    }

    /// Start a tutorial
    pub fn start_tutorial(&mut self, tutorial_id: &str) -> TutorialResult<()> {
        if !self.config.enabled {
            return Err(TutorialError::ConfigError("Tutorial system is disabled".to_string()));
        }

        if self.active_tutorial.is_some() {
            return Err(TutorialError::AlreadyActive(self.active_tutorial.clone().unwrap()));
        }

        let tutorial = self.tutorials.get(tutorial_id)
            .ok_or_else(|| TutorialError::TutorialNotFound(tutorial_id.to_string()))?;

        // Check prerequisites
        for prereq in &tutorial.prerequisites {
            if !self.completed_tutorials.contains(prereq) {
                return Err(TutorialError::PrerequisitesNotMet(tutorial.prerequisites.clone()));
            }
        }

        self.active_tutorial = Some(tutorial_id.to_string());
        self.current_step = Some(0);

        // Initialize progress
        if let Some(progress) = self.progress.get_mut(tutorial_id) {
            progress.start_time = 0.0; // This would be set to current time
            progress.completed = false;
            progress.skipped = false;
        }

        self.emit_event(TutorialEvent::Started(tutorial_id.to_string()));
        Ok(())
    }

    /// Complete current step
    pub fn complete_step(&mut self) -> TutorialResult<()> {
        let tutorial_id = self.active_tutorial.as_ref()
            .ok_or(TutorialError::NotActive)?;

        let current_step = self.current_step
            .ok_or(TutorialError::NotActive)?;

        let tutorial = self.tutorials.get(tutorial_id)
            .ok_or_else(|| TutorialError::TutorialNotFound(tutorial_id.clone()))?;

        if current_step >= tutorial.steps.len() {
            return Err(TutorialError::StepNotFound(tutorial_id.clone(), current_step));
        }

        // Update progress
        if let Some(progress) = self.progress.get_mut(tutorial_id) {
            progress.complete_step(current_step);
        }

        self.emit_event(TutorialEvent::StepCompleted(tutorial_id.clone(), current_step));

        // Check if tutorial is complete
        if current_step + 1 >= tutorial.steps.len() {
            self.complete_tutorial(tutorial_id)?;
        } else {
            self.current_step = Some(current_step + 1);
            self.emit_event(TutorialEvent::StepStarted(tutorial_id.clone(), current_step + 1));
        }

        Ok(())
    }

    /// Complete tutorial
    pub fn complete_tutorial(&mut self, tutorial_id: &str) -> TutorialResult<()> {
        if let Some(progress) = self.progress.get_mut(tutorial_id) {
            progress.completed = true;
            progress.completed_steps.clear();
            progress.completed_steps.extend(0..self.tutorials.get(tutorial_id).unwrap().steps.len());
        }

        self.completed_tutorials.insert(tutorial_id.to_string());
        self.active_tutorial = None;
        self.current_step = None;

        // Update statistics
        let completion_time = 0.0; // This would be calculated
        self.stats.update(tutorial_id, completion_time, false);

        self.emit_event(TutorialEvent::Completed(tutorial_id.to_string()));
        Ok(())
    }

    /// Skip tutorial
    pub fn skip_tutorial(&mut self) -> TutorialResult<()> {
        let tutorial_id = self.active_tutorial.as_ref()
            .ok_or(TutorialError::NotActive)?;

        if let Some(progress) = self.progress.get_mut(tutorial_id) {
            progress.skip();
        }

        self.completed_tutorials.insert(tutorial_id.clone());
        self.active_tutorial = None;
        self.current_step = None;

        // Update statistics
        let completion_time = 0.0; // This would be calculated
        self.stats.update(tutorial_id, completion_time, true);

        self.emit_event(TutorialEvent::Skipped(tutorial_id.clone()));
        Ok(())
    }

    /// Pause tutorial
    pub fn pause_tutorial(&mut self) -> TutorialResult<()> {
        let tutorial_id = self.active_tutorial.as_ref()
            .ok_or(TutorialError::NotActive)?;

        self.emit_event(TutorialEvent::Paused(tutorial_id.clone()));
        Ok(())
    }

    /// Resume tutorial
    pub fn resume_tutorial(&mut self) -> TutorialResult<()> {
        let tutorial_id = self.active_tutorial.as_ref()
            .ok_or(TutorialError::NotActive)?;

        self.emit_event(TutorialEvent::Resumed(tutorial_id.clone()));
        Ok(())
    }

    /// Get current tutorial
    pub fn get_current_tutorial(&self) -> Option<&Tutorial> {
        self.active_tutorial.as_ref()
            .and_then(|id| self.tutorials.get(id))
    }

    /// Get current step
    pub fn get_current_step(&self) -> Option<&TutorialStep> {
        self.get_current_tutorial()
            .and_then(|tutorial| {
                self.current_step.and_then(|step_index| tutorial.steps.get(step_index))
            })
    }

    /// Check if tutorial is active
    pub fn is_tutorial_active(&self) -> bool {
        self.active_tutorial.is_some()
    }

    /// Check if tutorial is completed
    pub fn is_tutorial_completed(&self, tutorial_id: &str) -> bool {
        self.completed_tutorials.contains(tutorial_id)
    }

    /// Get tutorial progress
    pub fn get_tutorial_progress(&self, tutorial_id: &str) -> Option<&TutorialProgress> {
        self.progress.get(tutorial_id)
    }

    /// Update tutorial system
    pub fn update(&mut self, delta_time: f32, context: &TutorialContext) {
        if !self.config.enabled {
            return;
        }

        // Update UI
        self.ui.update(delta_time);

        // Update current tutorial
        if let Some(tutorial_id) = &self.active_tutorial {
            if let Some(progress) = self.progress.get_mut(tutorial_id) {
                progress.update(delta_time, self.tutorials.get(tutorial_id).unwrap().steps.len());
            }

            // Check step completion
            if let Some(current_step) = self.current_step {
                if let Some(tutorial) = self.tutorials.get(tutorial_id) {
                    if let Some(step) = tutorial.steps.get(current_step) {
                        if step.completion_condition.evaluate(context) {
                            let _ = self.complete_step();
                        }
                    }
                }
            }
        }
    }

    /// Render tutorial UI
    pub fn render(&self) {
        if !self.config.enabled || !self.settings.show_ui {
            return;
        }

        self.ui.render();
    }

    /// Register event handler
    pub fn register_event_handler<F>(&mut self, event: TutorialEvent, handler: F)
    where
        F: Fn(&TutorialEvent) + Send + Sync + 'static,
    {
        self.event_handlers.entry(event).or_insert_with(Vec::new).push(Box::new(handler));
    }

    /// Emit event
    fn emit_event(&self, event: TutorialEvent) {
        if let Some(handlers) = self.event_handlers.get(&event) {
            for handler in handlers {
                handler(&event);
            }
        }
    }

    /// Show tutorial overlay
    pub fn show_overlay(&mut self, overlay: TutorialOverlay) {
        self.ui.show_overlay(overlay);
    }

    /// Hide tutorial overlay
    pub fn hide_overlay(&mut self, overlay_id: &str) {
        self.ui.hide_overlay(overlay_id);
    }

    /// Show highlight
    pub fn show_highlight(&mut self, highlight: TutorialHighlight) {
        let overlay = TutorialOverlay {
            id: highlight.id.clone(),
            overlay_type: super::ui::TutorialOverlayType::Highlight,
            position: (0.0, 0.0),
            size: (100.0, 100.0),
            visible: true,
            opacity: 1.0,
            animation: None,
            content: super::ui::TutorialOverlayContent::Highlight(highlight),
            duration: 0.0,
            age: 0.0,
        };
        self.show_overlay(overlay);
    }

    /// Hide highlight
    pub fn hide_highlight(&mut self, highlight_id: &str) {
        self.hide_overlay(highlight_id);
    }

    /// Get available tutorials
    pub fn get_available_tutorials(&self) -> Vec<&Tutorial> {
        self.tutorials.values()
            .filter(|tutorial| {
                // Check prerequisites
                tutorial.prerequisites.iter().all(|prereq| {
                    self.completed_tutorials.contains(prereq)
                })
            })
            .collect()
    }

    /// Get completed tutorials
    pub fn get_completed_tutorials(&self) -> Vec<&Tutorial> {
        self.completed_tutorials.iter()
            .filter_map(|id| self.tutorials.get(id))
            .collect()
    }

    /// Get tutorial statistics
    pub fn get_stats(&self) -> &TutorialStats {
        &self.stats
    }

    /// Reset tutorial progress
    pub fn reset_tutorial_progress(&mut self, tutorial_id: &str) -> TutorialResult<()> {
        if let Some(progress) = self.progress.get_mut(tutorial_id) {
            progress.current_step = 0;
            progress.completed_steps.clear();
            progress.completed = false;
            progress.skipped = false;
            progress.progress = 0.0;
        }

        self.completed_tutorials.remove(tutorial_id);
        Ok(())
    }

    /// Reset all tutorial progress
    pub fn reset_all_tutorial_progress(&mut self) {
        for progress in self.progress.values_mut() {
            progress.current_step = 0;
            progress.completed_steps.clear();
            progress.completed = false;
            progress.skipped = false;
            progress.progress = 0.0;
        }

        self.completed_tutorials.clear();
        self.active_tutorial = None;
        self.current_step = None;
    }

    /// Enable tutorial system
    pub fn enable(&mut self) {
        self.config.enabled = true;
    }

    /// Disable tutorial system
    pub fn disable(&mut self) {
        self.config.enabled = false;
        self.active_tutorial = None;
        self.current_step = None;
    }

    /// Set configuration
    pub fn set_config(&mut self, config: TutorialConfig) {
        self.config = config;
    }

    /// Set settings
    pub fn set_settings(&mut self, settings: TutorialSettings) {
        self.settings = settings;
    }
}

impl Default for TutorialManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Tutorial builder for creating tutorials
pub struct TutorialBuilder {
    tutorial: Tutorial,
}

impl TutorialBuilder {
    /// Create a new tutorial builder
    pub fn new(id: String, name: String, description: String) -> Self {
        Self {
            tutorial: Tutorial {
                id,
                name,
                description,
                steps: Vec::new(),
                prerequisites: Vec::new(),
                rewards: Vec::new(),
            },
        }
    }

    /// Add prerequisite
    pub fn add_prerequisite(mut self, prerequisite: String) -> Self {
        self.tutorial.prerequisites.push(prerequisite);
        self
    }

    /// Add reward
    pub fn add_reward(mut self, reward: super::TutorialReward) -> Self {
        self.tutorial.rewards.push(reward);
        self
    }

    /// Add step
    pub fn add_step(mut self, step: TutorialStep) -> Self {
        self.tutorial.steps.push(step);
        self
    }

    /// Build the tutorial
    pub fn build(self) -> Tutorial {
        self.tutorial
    }
}
