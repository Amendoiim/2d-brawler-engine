//! Fallback system for localization
//! 
//! This module provides fallback mechanisms when translations are missing
//! or incomplete.

use super::{Language, StringId, Translation, LocalizationError, LocalizationResult};
use crate::string_id;
use std::collections::HashMap;

/// Fallback strategy for handling missing translations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FallbackStrategy {
    /// Use the string ID as the fallback text
    UseStringId,
    /// Use a placeholder text
    UsePlaceholder,
    /// Use a specific fallback language
    UseLanguage(Language),
    /// Use multiple fallback languages in order
    UseLanguageChain(Vec<Language>),
    /// Don't show anything (empty string)
    Hide,
}

/// Fallback manager for handling missing translations
pub struct FallbackManager {
    /// Fallback strategy
    strategy: FallbackStrategy,
    /// Placeholder text for missing translations
    placeholder: String,
    /// Fallback language chain
    language_chain: Vec<Language>,
    /// Fallback translations
    fallback_translations: HashMap<StringId, String>,
}

impl FallbackManager {
    /// Create a new fallback manager
    pub fn new(strategy: FallbackStrategy) -> Self {
        Self {
            strategy,
            placeholder: "[MISSING TRANSLATION]".to_string(),
            language_chain: vec![Language::English],
            fallback_translations: HashMap::new(),
        }
    }

    /// Set the placeholder text
    pub fn set_placeholder(&mut self, placeholder: String) {
        self.placeholder = placeholder;
    }

    /// Set the fallback language chain
    pub fn set_language_chain(&mut self, chain: Vec<Language>) {
        self.language_chain = chain;
    }

    /// Add a fallback translation
    pub fn add_fallback_translation(&mut self, id: StringId, text: String) {
        self.fallback_translations.insert(id, text);
    }

    /// Get fallback text for a string ID
    pub fn get_fallback_text(&self, id: &StringId, available_languages: &[Language]) -> String {
        match self.strategy {
            FallbackStrategy::UseStringId => id.as_str().to_string(),
            FallbackStrategy::UsePlaceholder => self.placeholder.clone(),
            FallbackStrategy::UseLanguage(lang) => {
                if available_languages.contains(&lang) {
                    format!("[{}] {}", lang.code(), id.as_str())
                } else {
                    id.as_str().to_string()
                }
            }
            FallbackStrategy::UseLanguageChain(ref chain) => {
                for lang in chain {
                    if available_languages.contains(lang) {
                        return format!("[{}] {}", lang.code(), id.as_str());
                    }
                }
                id.as_str().to_string()
            }
            FallbackStrategy::Hide => String::new(),
        }
    }

    /// Check if a fallback translation exists
    pub fn has_fallback_translation(&self, id: &StringId) -> bool {
        self.fallback_translations.contains_key(id)
    }

    /// Get a fallback translation
    pub fn get_fallback_translation(&self, id: &StringId) -> Option<&String> {
        self.fallback_translations.get(id)
    }
}

/// Default fallback manager with sensible defaults
impl Default for FallbackManager {
    fn default() -> Self {
        Self::new(FallbackStrategy::UseStringId)
    }
}

/// Fallback text generator for common game elements
pub struct FallbackTextGenerator;

impl FallbackTextGenerator {
    /// Generate fallback text for UI elements
    pub fn generate_ui_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        // Common UI strings
        fallbacks.insert(string_id!("ui.main_menu"), "Main Menu".to_string());
        fallbacks.insert(string_id!("ui.play"), "Play".to_string());
        fallbacks.insert(string_id!("ui.settings"), "Settings".to_string());
        fallbacks.insert(string_id!("ui.quit"), "Quit".to_string());
        fallbacks.insert(string_id!("ui.back"), "Back".to_string());
        fallbacks.insert(string_id!("ui.next"), "Next".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "Cancel".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "Confirm".to_string());
        fallbacks.insert(string_id!("ui.save"), "Save".to_string());
        fallbacks.insert(string_id!("ui.load"), "Load".to_string());
        fallbacks.insert(string_id!("ui.pause"), "Pause".to_string());
        fallbacks.insert(string_id!("ui.resume"), "Resume".to_string());
        fallbacks.insert(string_id!("ui.exit"), "Exit".to_string());
        
        fallbacks
    }

    /// Generate fallback text for gameplay elements
    pub fn generate_gameplay_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        // Common gameplay strings
        fallbacks.insert(string_id!("gameplay.level"), "Level".to_string());
        fallbacks.insert(string_id!("gameplay.score"), "Score".to_string());
        fallbacks.insert(string_id!("gameplay.time"), "Time".to_string());
        fallbacks.insert(string_id!("gameplay.health"), "Health".to_string());
        fallbacks.insert(string_id!("gameplay.mana"), "Mana".to_string());
        fallbacks.insert(string_id!("gameplay.stamina"), "Stamina".to_string());
        fallbacks.insert(string_id!("gameplay.experience"), "Experience".to_string());
        fallbacks.insert(string_id!("gameplay.gold"), "Gold".to_string());
        fallbacks.insert(string_id!("gameplay.inventory"), "Inventory".to_string());
        fallbacks.insert(string_id!("gameplay.equipment"), "Equipment".to_string());
        fallbacks.insert(string_id!("gameplay.skills"), "Skills".to_string());
        fallbacks.insert(string_id!("gameplay.abilities"), "Abilities".to_string());
        
        fallbacks
    }

    /// Generate fallback text for combat elements
    pub fn generate_combat_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        // Common combat strings
        fallbacks.insert(string_id!("combat.attack"), "Attack".to_string());
        fallbacks.insert(string_id!("combat.defend"), "Defend".to_string());
        fallbacks.insert(string_id!("combat.dodge"), "Dodge".to_string());
        fallbacks.insert(string_id!("combat.block"), "Block".to_string());
        fallbacks.insert(string_id!("combat.parry"), "Parry".to_string());
        fallbacks.insert(string_id!("combat.critical_hit"), "Critical Hit!".to_string());
        fallbacks.insert(string_id!("combat.miss"), "Miss".to_string());
        fallbacks.insert(string_id!("combat.damage"), "Damage".to_string());
        fallbacks.insert(string_id!("combat.heal"), "Heal".to_string());
        fallbacks.insert(string_id!("combat.death"), "Death".to_string());
        fallbacks.insert(string_id!("combat.victory"), "Victory!".to_string());
        fallbacks.insert(string_id!("combat.defeat"), "Defeat".to_string());
        
        fallbacks
    }

    /// Generate fallback text for error messages
    pub fn generate_error_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        // Common error strings
        fallbacks.insert(string_id!("errors.generic"), "An error occurred".to_string());
        fallbacks.insert(string_id!("errors.save_failed"), "Failed to save game".to_string());
        fallbacks.insert(string_id!("errors.load_failed"), "Failed to load game".to_string());
        fallbacks.insert(string_id!("errors.network"), "Network error".to_string());
        fallbacks.insert(string_id!("errors.invalid_input"), "Invalid input".to_string());
        fallbacks.insert(string_id!("errors.file_not_found"), "File not found".to_string());
        fallbacks.insert(string_id!("errors.permission_denied"), "Permission denied".to_string());
        fallbacks.insert(string_id!("errors.out_of_memory"), "Out of memory".to_string());
        
        fallbacks
    }

    /// Generate all fallback texts
    pub fn generate_all_fallbacks() -> HashMap<StringId, String> {
        let mut all_fallbacks = HashMap::new();
        
        all_fallbacks.extend(Self::generate_ui_fallbacks());
        all_fallbacks.extend(Self::generate_gameplay_fallbacks());
        all_fallbacks.extend(Self::generate_combat_fallbacks());
        all_fallbacks.extend(Self::generate_error_fallbacks());
        
        all_fallbacks
    }
}

/// Fallback text for specific languages
pub struct LanguageFallbackText;

impl LanguageFallbackText {
    /// Get fallback text for a specific language
    pub fn get_language_fallbacks(language: Language) -> HashMap<StringId, String> {
        match language {
            Language::English => Self::get_english_fallbacks(),
            Language::French => Self::get_french_fallbacks(),
            Language::Italian => Self::get_italian_fallbacks(),
            Language::German => Self::get_german_fallbacks(),
            Language::Spanish => Self::get_spanish_fallbacks(),
            Language::BrazilianPortuguese => Self::get_portuguese_fallbacks(),
            Language::Korean => Self::get_korean_fallbacks(),
            Language::Japanese => Self::get_japanese_fallbacks(),
            Language::ChineseSimplified => Self::get_chinese_simplified_fallbacks(),
            Language::ChineseTraditional => Self::get_chinese_traditional_fallbacks(),
            Language::Turkish => Self::get_turkish_fallbacks(),
            Language::Arabic => Self::get_arabic_fallbacks(),
        }
    }

    /// English fallbacks (default)
    fn get_english_fallbacks() -> HashMap<StringId, String> {
        FallbackTextGenerator::generate_all_fallbacks()
    }

    /// French fallbacks
    fn get_french_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "Menu Principal".to_string());
        fallbacks.insert(string_id!("ui.play"), "Jouer".to_string());
        fallbacks.insert(string_id!("ui.settings"), "Paramètres".to_string());
        fallbacks.insert(string_id!("ui.quit"), "Quitter".to_string());
        fallbacks.insert(string_id!("ui.back"), "Retour".to_string());
        fallbacks.insert(string_id!("ui.next"), "Suivant".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "Annuler".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "Confirmer".to_string());
        fallbacks.insert(string_id!("ui.save"), "Sauvegarder".to_string());
        fallbacks.insert(string_id!("ui.load"), "Charger".to_string());
        fallbacks.insert(string_id!("ui.pause"), "Pause".to_string());
        fallbacks.insert(string_id!("ui.resume"), "Reprendre".to_string());
        fallbacks.insert(string_id!("ui.exit"), "Sortir".to_string());
        
        fallbacks
    }

    /// Italian fallbacks
    fn get_italian_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "Menu Principale".to_string());
        fallbacks.insert(string_id!("ui.play"), "Gioca".to_string());
        fallbacks.insert(string_id!("ui.settings"), "Impostazioni".to_string());
        fallbacks.insert(string_id!("ui.quit"), "Esci".to_string());
        fallbacks.insert(string_id!("ui.back"), "Indietro".to_string());
        fallbacks.insert(string_id!("ui.next"), "Avanti".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "Annulla".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "Conferma".to_string());
        fallbacks.insert(string_id!("ui.save"), "Salva".to_string());
        fallbacks.insert(string_id!("ui.load"), "Carica".to_string());
        fallbacks.insert(string_id!("ui.pause"), "Pausa".to_string());
        fallbacks.insert(string_id!("ui.resume"), "Riprendi".to_string());
        fallbacks.insert(string_id!("ui.exit"), "Esci".to_string());
        
        fallbacks
    }

    /// German fallbacks
    fn get_german_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "Hauptmenü".to_string());
        fallbacks.insert(string_id!("ui.play"), "Spielen".to_string());
        fallbacks.insert(string_id!("ui.settings"), "Einstellungen".to_string());
        fallbacks.insert(string_id!("ui.quit"), "Beenden".to_string());
        fallbacks.insert(string_id!("ui.back"), "Zurück".to_string());
        fallbacks.insert(string_id!("ui.next"), "Weiter".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "Abbrechen".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "Bestätigen".to_string());
        fallbacks.insert(string_id!("ui.save"), "Speichern".to_string());
        fallbacks.insert(string_id!("ui.load"), "Laden".to_string());
        fallbacks.insert(string_id!("ui.pause"), "Pause".to_string());
        fallbacks.insert(string_id!("ui.resume"), "Fortsetzen".to_string());
        fallbacks.insert(string_id!("ui.exit"), "Beenden".to_string());
        
        fallbacks
    }

    /// Spanish fallbacks
    fn get_spanish_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "Menú Principal".to_string());
        fallbacks.insert(string_id!("ui.play"), "Jugar".to_string());
        fallbacks.insert(string_id!("ui.settings"), "Configuración".to_string());
        fallbacks.insert(string_id!("ui.quit"), "Salir".to_string());
        fallbacks.insert(string_id!("ui.back"), "Atrás".to_string());
        fallbacks.insert(string_id!("ui.next"), "Siguiente".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "Cancelar".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "Confirmar".to_string());
        fallbacks.insert(string_id!("ui.save"), "Guardar".to_string());
        fallbacks.insert(string_id!("ui.load"), "Cargar".to_string());
        fallbacks.insert(string_id!("ui.pause"), "Pausa".to_string());
        fallbacks.insert(string_id!("ui.resume"), "Continuar".to_string());
        fallbacks.insert(string_id!("ui.exit"), "Salir".to_string());
        
        fallbacks
    }

    /// Portuguese fallbacks
    fn get_portuguese_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "Menu Principal".to_string());
        fallbacks.insert(string_id!("ui.play"), "Jogar".to_string());
        fallbacks.insert(string_id!("ui.settings"), "Configurações".to_string());
        fallbacks.insert(string_id!("ui.quit"), "Sair".to_string());
        fallbacks.insert(string_id!("ui.back"), "Voltar".to_string());
        fallbacks.insert(string_id!("ui.next"), "Próximo".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "Cancelar".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "Confirmar".to_string());
        fallbacks.insert(string_id!("ui.save"), "Salvar".to_string());
        fallbacks.insert(string_id!("ui.load"), "Carregar".to_string());
        fallbacks.insert(string_id!("ui.pause"), "Pausar".to_string());
        fallbacks.insert(string_id!("ui.resume"), "Continuar".to_string());
        fallbacks.insert(string_id!("ui.exit"), "Sair".to_string());
        
        fallbacks
    }

    /// Korean fallbacks
    fn get_korean_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "메인 메뉴".to_string());
        fallbacks.insert(string_id!("ui.play"), "플레이".to_string());
        fallbacks.insert(string_id!("ui.settings"), "설정".to_string());
        fallbacks.insert(string_id!("ui.quit"), "종료".to_string());
        fallbacks.insert(string_id!("ui.back"), "뒤로".to_string());
        fallbacks.insert(string_id!("ui.next"), "다음".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "취소".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "확인".to_string());
        fallbacks.insert(string_id!("ui.save"), "저장".to_string());
        fallbacks.insert(string_id!("ui.load"), "불러오기".to_string());
        fallbacks.insert(string_id!("ui.pause"), "일시정지".to_string());
        fallbacks.insert(string_id!("ui.resume"), "계속".to_string());
        fallbacks.insert(string_id!("ui.exit"), "종료".to_string());
        
        fallbacks
    }

    /// Japanese fallbacks
    fn get_japanese_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "メインメニュー".to_string());
        fallbacks.insert(string_id!("ui.play"), "プレイ".to_string());
        fallbacks.insert(string_id!("ui.settings"), "設定".to_string());
        fallbacks.insert(string_id!("ui.quit"), "終了".to_string());
        fallbacks.insert(string_id!("ui.back"), "戻る".to_string());
        fallbacks.insert(string_id!("ui.next"), "次へ".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "キャンセル".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "確認".to_string());
        fallbacks.insert(string_id!("ui.save"), "保存".to_string());
        fallbacks.insert(string_id!("ui.load"), "読み込み".to_string());
        fallbacks.insert(string_id!("ui.pause"), "一時停止".to_string());
        fallbacks.insert(string_id!("ui.resume"), "再開".to_string());
        fallbacks.insert(string_id!("ui.exit"), "終了".to_string());
        
        fallbacks
    }

    /// Chinese Simplified fallbacks
    fn get_chinese_simplified_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "主菜单".to_string());
        fallbacks.insert(string_id!("ui.play"), "开始游戏".to_string());
        fallbacks.insert(string_id!("ui.settings"), "设置".to_string());
        fallbacks.insert(string_id!("ui.quit"), "退出".to_string());
        fallbacks.insert(string_id!("ui.back"), "返回".to_string());
        fallbacks.insert(string_id!("ui.next"), "下一步".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "取消".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "确认".to_string());
        fallbacks.insert(string_id!("ui.save"), "保存".to_string());
        fallbacks.insert(string_id!("ui.load"), "加载".to_string());
        fallbacks.insert(string_id!("ui.pause"), "暂停".to_string());
        fallbacks.insert(string_id!("ui.resume"), "继续".to_string());
        fallbacks.insert(string_id!("ui.exit"), "退出".to_string());
        
        fallbacks
    }

    /// Chinese Traditional fallbacks
    fn get_chinese_traditional_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "主選單".to_string());
        fallbacks.insert(string_id!("ui.play"), "開始遊戲".to_string());
        fallbacks.insert(string_id!("ui.settings"), "設定".to_string());
        fallbacks.insert(string_id!("ui.quit"), "退出".to_string());
        fallbacks.insert(string_id!("ui.back"), "返回".to_string());
        fallbacks.insert(string_id!("ui.next"), "下一步".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "取消".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "確認".to_string());
        fallbacks.insert(string_id!("ui.save"), "儲存".to_string());
        fallbacks.insert(string_id!("ui.load"), "載入".to_string());
        fallbacks.insert(string_id!("ui.pause"), "暫停".to_string());
        fallbacks.insert(string_id!("ui.resume"), "繼續".to_string());
        fallbacks.insert(string_id!("ui.exit"), "退出".to_string());
        
        fallbacks
    }

    /// Turkish fallbacks
    fn get_turkish_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "Ana Menü".to_string());
        fallbacks.insert(string_id!("ui.play"), "Oyna".to_string());
        fallbacks.insert(string_id!("ui.settings"), "Ayarlar".to_string());
        fallbacks.insert(string_id!("ui.quit"), "Çıkış".to_string());
        fallbacks.insert(string_id!("ui.back"), "Geri".to_string());
        fallbacks.insert(string_id!("ui.next"), "İleri".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "İptal".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "Onayla".to_string());
        fallbacks.insert(string_id!("ui.save"), "Kaydet".to_string());
        fallbacks.insert(string_id!("ui.load"), "Yükle".to_string());
        fallbacks.insert(string_id!("ui.pause"), "Duraklat".to_string());
        fallbacks.insert(string_id!("ui.resume"), "Devam Et".to_string());
        fallbacks.insert(string_id!("ui.exit"), "Çıkış".to_string());
        
        fallbacks
    }

    /// Arabic fallbacks
    fn get_arabic_fallbacks() -> HashMap<StringId, String> {
        let mut fallbacks = HashMap::new();
        
        fallbacks.insert(string_id!("ui.main_menu"), "القائمة الرئيسية".to_string());
        fallbacks.insert(string_id!("ui.play"), "اللعب".to_string());
        fallbacks.insert(string_id!("ui.settings"), "الإعدادات".to_string());
        fallbacks.insert(string_id!("ui.quit"), "الخروج".to_string());
        fallbacks.insert(string_id!("ui.back"), "رجوع".to_string());
        fallbacks.insert(string_id!("ui.next"), "التالي".to_string());
        fallbacks.insert(string_id!("ui.cancel"), "إلغاء".to_string());
        fallbacks.insert(string_id!("ui.confirm"), "تأكيد".to_string());
        fallbacks.insert(string_id!("ui.save"), "حفظ".to_string());
        fallbacks.insert(string_id!("ui.load"), "تحميل".to_string());
        fallbacks.insert(string_id!("ui.pause"), "إيقاف مؤقت".to_string());
        fallbacks.insert(string_id!("ui.resume"), "متابعة".to_string());
        fallbacks.insert(string_id!("ui.exit"), "الخروج".to_string());
        
        fallbacks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_manager() {
        let manager = FallbackManager::new(FallbackStrategy::UseStringId);
        let id = string_id!("test.hello");
        let available_languages = vec![Language::English];
        
        let fallback_text = manager.get_fallback_text(&id, &available_languages);
        assert_eq!(fallback_text, "test.hello");
    }

    #[test]
    fn test_fallback_text_generator() {
        let fallbacks = FallbackTextGenerator::generate_ui_fallbacks();
        assert!(fallbacks.contains_key(&string_id!("ui.main_menu")));
        assert_eq!(fallbacks.get(&string_id!("ui.main_menu")), Some(&"Main Menu".to_string()));
    }

    #[test]
    fn test_language_fallback_text() {
        let english_fallbacks = LanguageFallbackText::get_language_fallbacks(Language::English);
        let french_fallbacks = LanguageFallbackText::get_language_fallbacks(Language::French);
        
        assert_eq!(english_fallbacks.get(&string_id!("ui.main_menu")), Some(&"Main Menu".to_string()));
        assert_eq!(french_fallbacks.get(&string_id!("ui.main_menu")), Some(&"Menu Principal".to_string()));
    }
}
