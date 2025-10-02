//! End Game Manager
//! 
//! This module manages end game content, final boss encounters, and game completion sequences.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

use super::{CompletionConfig, CompletionEvent, CompletionResult, CompletionError};

/// End game manager
pub struct EndGameManager {
    /// Configuration
    pub config: CompletionConfig,
    /// End game unlocked
    pub end_game_unlocked: bool,
    /// End game completed
    pub end_game_completed: bool,
    /// Final boss defeated
    pub final_boss_defeated: bool,
    /// End game sequences
    pub sequences: HashMap<String, EndGameSequence>,
    /// Current sequence
    pub current_sequence: Option<String>,
    /// Sequence progress
    pub sequence_progress: f32,
    /// Event handlers
    pub event_handlers: Vec<Box<dyn Fn(&CompletionEvent) + Send + Sync>>,
}

/// End game sequence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndGameSequence {
    /// Sequence ID
    pub id: String,
    /// Sequence name
    pub name: String,
    /// Sequence type
    pub sequence_type: EndGameSequenceType,
    /// Sequence steps
    pub steps: Vec<EndGameStep>,
    /// Current step index
    pub current_step: usize,
    /// Sequence completed
    pub completed: bool,
    /// Sequence duration
    pub duration: f32,
}

/// End game sequence type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EndGameSequenceType {
    /// Final boss battle
    FinalBoss,
    /// Cutscene sequence
    Cutscene,
    /// Interactive sequence
    Interactive,
    /// Credits sequence
    Credits,
    /// Epilogue sequence
    Epilogue,
}

/// End game step
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndGameStep {
    /// Step ID
    pub id: String,
    /// Step name
    pub name: String,
    /// Step type
    pub step_type: EndGameStepType,
    /// Step duration
    pub duration: f32,
    /// Step completed
    pub completed: bool,
    /// Step data
    pub data: HashMap<String, serde_json::Value>,
}

/// End game step type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EndGameStepType {
    /// Boss battle
    BossBattle,
    /// Cutscene
    Cutscene,
    /// Dialog
    Dialog,
    /// Action sequence
    Action,
    /// Transition
    Transition,
    /// Credits
    Credits,
}

impl EndGameManager {
    /// Create a new end game manager
    pub fn new(config: CompletionConfig) -> Self {
        let mut manager = Self {
            config,
            end_game_unlocked: false,
            end_game_completed: false,
            final_boss_defeated: false,
            sequences: HashMap::new(),
            current_sequence: None,
            sequence_progress: 0.0,
            event_handlers: Vec::new(),
        };

        // Initialize default sequences
        manager.initialize_sequences();
        manager
    }

    /// Update end game manager
    pub fn update(&mut self, delta_time: f32) -> CompletionResult<()> {
        if !self.config.end_game_enabled {
            return Ok(());
        }

        // Update current sequence
        if let Some(sequence_id) = self.current_sequence.clone() {
            self.update_sequence(&sequence_id, delta_time)?;
        }

        Ok(())
    }

    /// Unlock end game content
    pub fn unlock_end_game(&mut self) -> CompletionResult<()> {
        if !self.config.end_game_enabled {
            return Err(CompletionError::EndGameNotUnlocked);
        }

        self.end_game_unlocked = true;
        self.emit_event(CompletionEvent::EndGameUnlocked {
            completion_percentage: 80.0, // This would come from completion tracker
        });

        Ok(())
    }

    /// Start end game sequence
    pub fn start_sequence(&mut self, sequence_id: &str) -> CompletionResult<()> {
        if !self.end_game_unlocked {
            return Err(CompletionError::EndGameNotUnlocked);
        }

        if let Some(sequence) = self.sequences.get_mut(sequence_id) {
            sequence.current_step = 0;
            sequence.completed = false;
            self.current_sequence = Some(sequence_id.to_string());
            self.sequence_progress = 0.0;
            Ok(())
        } else {
            Err(CompletionError::Unknown(format!("Sequence not found: {}", sequence_id)))
        }
    }

    /// Complete current step
    pub fn complete_current_step(&mut self) -> CompletionResult<()> {
        if let Some(sequence_id) = self.current_sequence.clone() {
            if let Some(sequence) = self.sequences.get_mut(&sequence_id) {
                if sequence.current_step < sequence.steps.len() {
                    sequence.steps[sequence.current_step].completed = true;
                    sequence.current_step += 1;

                    // Check if sequence is completed
                    if sequence.current_step >= sequence.steps.len() {
                        sequence.completed = true;
                        self.complete_sequence(&sequence_id)?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Complete sequence
    fn complete_sequence(&mut self, sequence_id: &str) -> CompletionResult<()> {
        if let Some(sequence) = self.sequences.get(sequence_id) {
            match sequence.sequence_type {
                EndGameSequenceType::FinalBoss => {
                    self.final_boss_defeated = true;
                }
                EndGameSequenceType::Credits => {
                    self.emit_event(CompletionEvent::CreditsCompleted);
                }
                EndGameSequenceType::Epilogue => {
                    self.end_game_completed = true;
                }
                _ => {}
            }
        }

        self.current_sequence = None;
        self.sequence_progress = 0.0;

        Ok(())
    }

    /// Update sequence
    fn update_sequence(&mut self, sequence_id: &str, delta_time: f32) -> CompletionResult<()> {
        if let Some(sequence) = self.sequences.get_mut(sequence_id) {
            if sequence.current_step < sequence.steps.len() {
                let step = &mut sequence.steps[sequence.current_step];
                
                // Update step progress
                step.duration -= delta_time;
                
                // Check if step should auto-complete
                if step.duration <= 0.0 && !step.completed {
                    // Complete the step directly to avoid borrowing conflicts
                    step.completed = true;
                    sequence.current_step += 1;

                    // Check if sequence is completed
                    if sequence.current_step >= sequence.steps.len() {
                        sequence.completed = true;
                        self.complete_sequence(sequence_id)?;
                    }
                }
            }

            // Update sequence progress
            self.sequence_progress = sequence.current_step as f32 / sequence.steps.len() as f32;
        }

        Ok(())
    }

    /// Check if end game is unlocked
    pub fn is_end_game_unlocked(&self) -> bool {
        self.end_game_unlocked
    }

    /// Check if end game is completed
    pub fn is_end_game_completed(&self) -> bool {
        self.end_game_completed
    }

    /// Check if final boss is defeated
    pub fn is_final_boss_defeated(&self) -> bool {
        self.final_boss_defeated
    }

    /// Get current sequence
    pub fn get_current_sequence(&self) -> Option<&EndGameSequence> {
        if let Some(sequence_id) = &self.current_sequence {
            self.sequences.get(sequence_id)
        } else {
            None
        }
    }

    /// Get sequence progress
    pub fn get_sequence_progress(&self) -> f32 {
        self.sequence_progress
    }

    /// Get all sequences
    pub fn get_sequences(&self) -> &HashMap<String, EndGameSequence> {
        &self.sequences
    }

    /// Add event handler
    pub fn add_event_handler<F>(&mut self, handler: F)
    where
        F: Fn(&CompletionEvent) + Send + Sync + 'static,
    {
        self.event_handlers.push(Box::new(handler));
    }

    /// Initialize default sequences
    fn initialize_sequences(&mut self) {
        // Final boss sequence
        let final_boss_sequence = EndGameSequence {
            id: "final_boss".to_string(),
            name: "Final Boss Battle".to_string(),
            sequence_type: EndGameSequenceType::FinalBoss,
            steps: vec![
                EndGameStep {
                    id: "boss_intro".to_string(),
                    name: "Boss Introduction".to_string(),
                    step_type: EndGameStepType::Cutscene,
                    duration: 10.0,
                    completed: false,
                    data: HashMap::new(),
                },
                EndGameStep {
                    id: "boss_battle".to_string(),
                    name: "Boss Battle".to_string(),
                    step_type: EndGameStepType::BossBattle,
                    duration: 300.0, // 5 minutes
                    completed: false,
                    data: HashMap::new(),
                },
                EndGameStep {
                    id: "boss_defeat".to_string(),
                    name: "Boss Defeat".to_string(),
                    step_type: EndGameStepType::Cutscene,
                    duration: 15.0,
                    completed: false,
                    data: HashMap::new(),
                },
            ],
            current_step: 0,
            completed: false,
            duration: 325.0,
        };

        // Credits sequence
        let credits_sequence = EndGameSequence {
            id: "credits".to_string(),
            name: "Credits".to_string(),
            sequence_type: EndGameSequenceType::Credits,
            steps: vec![
                EndGameStep {
                    id: "credits_roll".to_string(),
                    name: "Credits Roll".to_string(),
                    step_type: EndGameStepType::Credits,
                    duration: 120.0, // 2 minutes
                    completed: false,
                    data: HashMap::new(),
                },
            ],
            current_step: 0,
            completed: false,
            duration: 120.0,
        };

        // Epilogue sequence
        let epilogue_sequence = EndGameSequence {
            id: "epilogue".to_string(),
            name: "Epilogue".to_string(),
            sequence_type: EndGameSequenceType::Epilogue,
            steps: vec![
                EndGameStep {
                    id: "epilogue_cutscene".to_string(),
                    name: "Epilogue Cutscene".to_string(),
                    step_type: EndGameStepType::Cutscene,
                    duration: 30.0,
                    completed: false,
                    data: HashMap::new(),
                },
                EndGameStep {
                    id: "epilogue_dialog".to_string(),
                    name: "Epilogue Dialog".to_string(),
                    step_type: EndGameStepType::Dialog,
                    duration: 60.0,
                    completed: false,
                    data: HashMap::new(),
                },
            ],
            current_step: 0,
            completed: false,
            duration: 90.0,
        };

        self.sequences.insert("final_boss".to_string(), final_boss_sequence);
        self.sequences.insert("credits".to_string(), credits_sequence);
        self.sequences.insert("epilogue".to_string(), epilogue_sequence);
    }

    /// Emit completion event
    fn emit_event(&self, event: CompletionEvent) {
        for handler in &self.event_handlers {
            handler(&event);
        }
    }
}

impl Default for EndGameManager {
    fn default() -> Self {
        Self::new(CompletionConfig::default())
    }
}
