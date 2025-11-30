use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::ability::{AbilityCheckRequest, AbilityChecker};
use crate::ability_scores::{AbilityScoreGenerator, AbilityScoreType, AbilityScores};
use crate::attack::{AttackRequest, AttackResolver};
use crate::cr_xp::{xp_to_cr_approximate, ChallengeRating};
use crate::damage::{DamageRequest, DamageResolver};
use crate::dice::{DiceRoller, RollMode};
use crate::error::{Result, RulesError};
use crate::skills::{Skill, SkillCalculator, SkillCheckResult};
use crate::spells::{Spell, SpellCastRequest, SpellCaster, SpellDatabase, SpellSchool};
use crate::weapons::{Weapon, WeaponCategory, WeaponDatabase, WeaponType};

#[derive(Clone)]
pub struct AppState {
    ability_checker: Arc<AbilityChecker>,
    attack_resolver: Arc<AttackResolver>,
    damage_resolver: Arc<DamageResolver>,
    spell_caster: Arc<SpellCaster>,
    spell_database: Arc<std::sync::Mutex<SpellDatabase>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct RollRequest {
    pub expression: String,
    pub seed: Option<u64>,
    pub advantage: Option<bool>,
    pub disadvantage: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct RollResponse {
    pub result: crate::dice::RollResult,
}

#[derive(Debug, Deserialize)]
pub struct SavingThrowRequest {
    pub ability: String,
    pub ability_modifier: i32,
    pub proficiency_bonus: i32,
    pub has_proficiency: bool,
    pub dc: i32,
    pub advantage: Option<bool>,
    pub disadvantage: Option<bool>,
    pub seed: Option<u64>,
}

pub struct RulesServer {
    state: AppState,
}

impl RulesServer {
    pub fn new() -> Result<Self> {
        Ok(Self {
            state: AppState {
                ability_checker: Arc::new(AbilityChecker::new()),
                attack_resolver: Arc::new(AttackResolver::new()),
                damage_resolver: Arc::new(DamageResolver::new()),
                spell_caster: Arc::new(SpellCaster::new()),
                spell_database: Arc::new(std::sync::Mutex::new(SpellDatabase::new())),
            },
        })
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/roll", post(roll_handler))
            .route("/attack", post(attack_handler))
            .route("/ability-check", post(ability_check_handler))
            .route("/saving-throw", post(saving_throw_handler))
            .route("/damage", post(damage_handler))
            .route(
                "/ability-scores/calculate-modifier",
                post(ability_modifier_handler),
            )
            .route(
                "/ability-scores/generate",
                post(generate_ability_scores_handler),
            )
            .route("/cr-xp/cr-to-xp", post(cr_to_xp_handler))
            .route("/cr-xp/xp-to-cr", post(xp_to_cr_handler))
            .route("/cr-xp/proficiency-bonus", post(cr_to_proficiency_handler))
            .route("/skills/list", get(list_skills_handler))
            .route(
                "/skills/calculate-bonus",
                post(calculate_skill_bonus_handler),
            )
            .route("/skills/check", post(skill_check_handler))
            .route(
                "/skills/passive-perception",
                post(passive_perception_handler),
            )
            .route("/weapons/list", get(list_weapons_handler))
            .route("/weapons/get/{weapon_name}", get(get_weapon_handler))
            .route(
                "/weapons/by-category",
                post(get_weapons_by_category_handler),
            )
            .route("/spells/list", get(list_spells_handler))
            .route("/spells/get/{spell_name}", get(get_spell_handler))
            .route("/spells/search", post(search_spells_handler))
            .route("/spells/by-level", post(get_spells_by_level_handler))
            .route("/spells/by-school", post(get_spells_by_school_handler))
            .route("/spells/by-class", post(get_spells_by_class_handler))
            .route("/spells/cast", post(cast_spell_handler))
            .route("/spells/slots/for-full-caster", post(create_full_caster_slots_handler))
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
            .with_state(self.state.clone());

        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(&addr).await.map_err(RulesError::Io)?;

        info!("Rules5e service listening on {}", addr);

        axum::serve(listener, app).await.map_err(RulesError::Io)?;

        Ok(())
    }
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "rules5e-service".to_string(),
        version: "1.0.0".to_string(),
    })
}

async fn roll_handler(
    State(_state): State<AppState>,
    Json(request): Json<RollRequest>,
) -> std::result::Result<Json<RollResponse>, (StatusCode, String)> {
    let expr = DiceRoller::parse(&request.expression).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Invalid dice expression: {}", e),
        )
    })?;

    let mode = if request.advantage.unwrap_or(false) && request.disadvantage.unwrap_or(false) {
        RollMode::Normal
    } else if request.advantage.unwrap_or(false) {
        RollMode::Advantage
    } else if request.disadvantage.unwrap_or(false) {
        RollMode::Disadvantage
    } else {
        RollMode::Normal
    };

    let mut roller = if let Some(seed) = request.seed {
        DiceRoller::with_seed(seed)
    } else {
        DiceRoller::new()
    };

    let result = roller.roll(&expr, mode).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Roll error: {}", e),
        )
    })?;

    Ok(Json(RollResponse { result }))
}

async fn attack_handler(
    State(state): State<AppState>,
    Json(request): Json<AttackRequest>,
) -> std::result::Result<Json<crate::attack::AttackResult>, (StatusCode, String)> {
    let result = state.attack_resolver.resolve(&request).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Attack error: {}", e),
        )
    })?;
    Ok(Json(result))
}

async fn ability_check_handler(
    State(state): State<AppState>,
    Json(request): Json<AbilityCheckRequest>,
) -> std::result::Result<Json<crate::ability::AbilityCheckResult>, (StatusCode, String)> {
    let result = state.ability_checker.check(&request).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Ability check error: {}", e),
        )
    })?;
    Ok(Json(result))
}

async fn saving_throw_handler(
    State(state): State<AppState>,
    Json(request): Json<SavingThrowRequest>,
) -> std::result::Result<Json<crate::ability::AbilityCheckResult>, (StatusCode, String)> {
    // Parse ability string to enum
    let ability = match request.ability.to_lowercase().as_str() {
        "strength" | "str" => crate::ability::Ability::Strength,
        "dexterity" | "dex" => crate::ability::Ability::Dexterity,
        "constitution" | "con" => crate::ability::Ability::Constitution,
        "intelligence" | "int" => crate::ability::Ability::Intelligence,
        "wisdom" | "wis" => crate::ability::Ability::Wisdom,
        "charisma" | "cha" => crate::ability::Ability::Charisma,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid ability: {}", request.ability),
            ))
        }
    };

    let ability_request = AbilityCheckRequest {
        ability,
        ability_modifier: request.ability_modifier,
        proficiency_bonus: request.proficiency_bonus,
        has_proficiency: request.has_proficiency,
        has_expertise: false,
        dc: request.dc,
        advantage: request.advantage.unwrap_or(false),
        disadvantage: request.disadvantage.unwrap_or(false),
        seed: request.seed,
    };

    let result = state.ability_checker.check(&ability_request).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Saving throw error: {}", e),
        )
    })?;
    Ok(Json(result))
}

async fn damage_handler(
    State(state): State<AppState>,
    Json(request): Json<DamageRequest>,
) -> Json<crate::damage::DamageResult> {
    let result = state.damage_resolver.resolve(&request);
    Json(result)
}

#[derive(Debug, Deserialize)]
pub struct AbilityModifierRequest {
    pub ability_score: u8,
}

#[derive(Debug, Serialize)]
pub struct AbilityModifierResponse {
    pub score: u8,
    pub modifier: i32,
}

async fn ability_modifier_handler(
    Json(request): Json<AbilityModifierRequest>,
) -> std::result::Result<Json<AbilityModifierResponse>, (StatusCode, String)> {
    if request.ability_score > 30 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Ability score cannot exceed 30".to_string(),
        ));
    }
    let modifier = (request.ability_score as i32 - 10) / 2;
    Ok(Json(AbilityModifierResponse {
        score: request.ability_score,
        modifier,
    }))
}

#[derive(Debug, Deserialize)]
pub struct GenerateAbilityScoresRequest {
    pub method: String, // "standard", "point_buy", "rolling"
    pub seed: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct GenerateAbilityScoresResponse {
    pub scores: AbilityScores,
    pub modifiers: AbilityScoreModifiers,
}

#[derive(Debug, Serialize)]
pub struct AbilityScoreModifiers {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
}

async fn generate_ability_scores_handler(
    Json(request): Json<GenerateAbilityScoresRequest>,
) -> std::result::Result<Json<GenerateAbilityScoresResponse>, (StatusCode, String)> {
    let scores = match request.method.to_lowercase().as_str() {
        "standard" | "standard_array" => AbilityScoreGenerator::generate_standard_array(),
        "point_buy" | "pointbuy" => AbilityScoreGenerator::generate_point_buy(),
        "rolling" | "roll" => AbilityScoreGenerator::generate_rolling(request.seed),
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid generation method: {}", request.method),
            ))
        }
    };

    let modifiers = AbilityScoreModifiers {
        strength: scores.get_modifier(AbilityScoreType::Strength),
        dexterity: scores.get_modifier(AbilityScoreType::Dexterity),
        constitution: scores.get_modifier(AbilityScoreType::Constitution),
        intelligence: scores.get_modifier(AbilityScoreType::Intelligence),
        wisdom: scores.get_modifier(AbilityScoreType::Wisdom),
        charisma: scores.get_modifier(AbilityScoreType::Charisma),
    };

    Ok(Json(GenerateAbilityScoresResponse { scores, modifiers }))
}

#[derive(Debug, Deserialize)]
pub struct CrToXpRequest {
    pub cr: String,
}

#[derive(Debug, Serialize)]
pub struct CrToXpResponse {
    pub cr: String,
    pub xp: u32,
    pub proficiency_bonus: i32,
}

async fn cr_to_xp_handler(
    Json(request): Json<CrToXpRequest>,
) -> std::result::Result<Json<CrToXpResponse>, (StatusCode, String)> {
    let cr = ChallengeRating::from_str(&request.cr)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid CR: {}", e)))?;
    let xp = cr.to_xp();
    let proficiency = cr.to_proficiency_bonus();
    Ok(Json(CrToXpResponse {
        cr: cr.to_string(),
        xp,
        proficiency_bonus: proficiency,
    }))
}

#[derive(Debug, Deserialize)]
pub struct XpToCrRequest {
    pub xp: u32,
}

#[derive(Debug, Serialize)]
pub struct XpToCrResponse {
    pub xp: u32,
    pub cr: String,
    pub approximate: bool,
}

async fn xp_to_cr_handler(Json(request): Json<XpToCrRequest>) -> Json<XpToCrResponse> {
    let cr = xp_to_cr_approximate(request.xp);
    Json(XpToCrResponse {
        xp: request.xp,
        cr: cr.to_string(),
        approximate: true,
    })
}

#[derive(Debug, Deserialize)]
pub struct CrToProficiencyRequest {
    pub cr: String,
}

#[derive(Debug, Serialize)]
pub struct CrToProficiencyResponse {
    pub cr: String,
    pub proficiency_bonus: i32,
}

async fn cr_to_proficiency_handler(
    Json(request): Json<CrToProficiencyRequest>,
) -> std::result::Result<Json<CrToProficiencyResponse>, (StatusCode, String)> {
    let cr = ChallengeRating::from_str(&request.cr)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid CR: {}", e)))?;
    Ok(Json(CrToProficiencyResponse {
        cr: cr.to_string(),
        proficiency_bonus: cr.to_proficiency_bonus(),
    }))
}

#[derive(Debug, Serialize)]
pub struct ListSkillsResponse {
    pub skills: Vec<SkillInfo>,
}

#[derive(Debug, Serialize)]
pub struct SkillInfo {
    pub name: String,
    pub ability: String,
}

async fn list_skills_handler() -> Json<ListSkillsResponse> {
    let skills: Vec<SkillInfo> = Skill::all()
        .into_iter()
        .map(|skill| {
            let ability = match skill.associated_ability() {
                AbilityScoreType::Strength => "Strength",
                AbilityScoreType::Dexterity => "Dexterity",
                AbilityScoreType::Constitution => "Constitution",
                AbilityScoreType::Intelligence => "Intelligence",
                AbilityScoreType::Wisdom => "Wisdom",
                AbilityScoreType::Charisma => "Charisma",
            };
            SkillInfo {
                name: skill.name().to_string(),
                ability: ability.to_string(),
            }
        })
        .collect();
    Json(ListSkillsResponse { skills })
}

#[derive(Debug, Deserialize)]
pub struct CalculateSkillBonusRequest {
    pub ability_scores: AbilityScores,
    pub skill: String,
    pub proficiency_bonus: i32,
    pub has_proficiency: bool,
    pub has_expertise: bool,
}

#[derive(Debug, Serialize)]
pub struct CalculateSkillBonusResponse {
    pub bonus: crate::skills::SkillBonus,
}

async fn calculate_skill_bonus_handler(
    Json(request): Json<CalculateSkillBonusRequest>,
) -> std::result::Result<Json<CalculateSkillBonusResponse>, (StatusCode, String)> {
    let skill = parse_skill(&request.skill)?;
    let bonus = SkillCalculator::calculate_skill_bonus(
        &request.ability_scores,
        skill,
        request.proficiency_bonus,
        request.has_proficiency,
        request.has_expertise,
    );
    Ok(Json(CalculateSkillBonusResponse { bonus }))
}

#[derive(Debug, Deserialize)]
pub struct SkillCheckRequestHttp {
    pub skill: String,
    pub ability_scores: AbilityScores,
    pub proficiency_bonus: i32,
    pub has_proficiency: bool,
    pub has_expertise: bool,
    pub dc: i32,
    pub advantage: bool,
    pub disadvantage: bool,
    pub seed: Option<u64>,
}

async fn skill_check_handler(
    State(state): State<AppState>,
    Json(request): Json<SkillCheckRequestHttp>,
) -> std::result::Result<Json<SkillCheckResult>, (StatusCode, String)> {
    let skill = parse_skill(&request.skill)?;
    let ability_type = skill.associated_ability();
    let ability_modifier = request.ability_scores.get_modifier(ability_type);

    let prof_bonus = if request.has_expertise {
        request.proficiency_bonus * 2
    } else if request.has_proficiency {
        request.proficiency_bonus
    } else {
        0
    };

    let ability_check_request = AbilityCheckRequest {
        ability: match ability_type {
            AbilityScoreType::Strength => crate::ability::Ability::Strength,
            AbilityScoreType::Dexterity => crate::ability::Ability::Dexterity,
            AbilityScoreType::Constitution => crate::ability::Ability::Constitution,
            AbilityScoreType::Intelligence => crate::ability::Ability::Intelligence,
            AbilityScoreType::Wisdom => crate::ability::Ability::Wisdom,
            AbilityScoreType::Charisma => crate::ability::Ability::Charisma,
        },
        ability_modifier,
        proficiency_bonus: prof_bonus,
        has_proficiency: request.has_proficiency,
        has_expertise: request.has_expertise,
        dc: request.dc,
        advantage: request.advantage,
        disadvantage: request.disadvantage,
        seed: request.seed,
    };

    let ability_result = state
        .ability_checker
        .check(&ability_check_request)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Skill check error: {}", e),
            )
        })?;

    let skill_result = SkillCheckResult::from_ability_check(
        skill,
        &request.ability_scores,
        request.proficiency_bonus,
        request.has_proficiency,
        request.has_expertise,
        ability_result,
    );

    Ok(Json(skill_result))
}

#[derive(Debug, Deserialize)]
pub struct PassivePerceptionRequest {
    pub ability_scores: AbilityScores,
    pub proficiency_bonus: i32,
    pub has_proficiency: bool,
    pub has_expertise: bool,
}

#[derive(Debug, Serialize)]
pub struct PassivePerceptionResponse {
    pub passive_perception: i32,
}

async fn passive_perception_handler(
    Json(request): Json<PassivePerceptionRequest>,
) -> Json<PassivePerceptionResponse> {
    let passive = SkillCalculator::passive_perception(
        &request.ability_scores,
        request.proficiency_bonus,
        request.has_proficiency,
        request.has_expertise,
    );
    Json(PassivePerceptionResponse {
        passive_perception: passive,
    })
}

fn parse_skill(skill_str: &str) -> std::result::Result<Skill, (StatusCode, String)> {
    match skill_str
        .to_lowercase()
        .replace(" ", "")
        .replace("_", "")
        .as_str()
    {
        "athletics" => Ok(Skill::Athletics),
        "acrobatics" => Ok(Skill::Acrobatics),
        "sleightofhand" | "sleight_of_hand" => Ok(Skill::SleightOfHand),
        "stealth" => Ok(Skill::Stealth),
        "arcana" => Ok(Skill::Arcana),
        "history" => Ok(Skill::History),
        "investigation" => Ok(Skill::Investigation),
        "nature" => Ok(Skill::Nature),
        "religion" => Ok(Skill::Religion),
        "animalhandling" | "animal_handling" => Ok(Skill::AnimalHandling),
        "insight" => Ok(Skill::Insight),
        "medicine" => Ok(Skill::Medicine),
        "perception" => Ok(Skill::Perception),
        "survival" => Ok(Skill::Survival),
        "deception" => Ok(Skill::Deception),
        "intimidation" => Ok(Skill::Intimidation),
        "performance" => Ok(Skill::Performance),
        "persuasion" => Ok(Skill::Persuasion),
        _ => Err((
            StatusCode::BAD_REQUEST,
            format!("Invalid skill: {}", skill_str),
        )),
    }
}

// Weapons handlers

#[derive(Debug, Serialize)]
pub struct ListWeaponsResponse {
    pub weapons: Vec<WeaponInfo>,
}

#[derive(Debug, Serialize)]
pub struct WeaponInfo {
    pub name: String,
    pub category: String,
    pub weapon_type: String,
    pub damage: String,
    pub damage_type: String,
    pub properties: Vec<String>,
}

async fn list_weapons_handler() -> Json<ListWeaponsResponse> {
    let weapons: Vec<WeaponInfo> = WeaponDatabase::all_weapons()
        .into_iter()
        .map(|w| {
            let category = match w.category {
                WeaponCategory::Simple => "Simple",
                WeaponCategory::Martial => "Martial",
            };
            let weapon_type = match w.weapon_type {
                WeaponType::Melee => "Melee",
                WeaponType::Ranged => "Ranged",
            };
            let damage = format!(
                "{}d{}{}",
                w.damage.count,
                w.damage.sides,
                if w.damage.modifier > 0 {
                    format!("+{}", w.damage.modifier)
                } else {
                    String::new()
                }
            );
            let damage_type = format!("{:?}", w.damage_type);
            let properties: Vec<String> = w.properties.iter().map(|p| format!("{:?}", p)).collect();

            WeaponInfo {
                name: w.name,
                category: category.to_string(),
                weapon_type: weapon_type.to_string(),
                damage,
                damage_type,
                properties,
            }
        })
        .collect();
    Json(ListWeaponsResponse { weapons })
}

async fn get_weapon_handler(
    axum::extract::Path(weapon_name): axum::extract::Path<String>,
) -> std::result::Result<Json<Weapon>, (StatusCode, String)> {
    WeaponDatabase::get_weapon(&weapon_name)
        .map(Json)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("Weapon not found: {}", weapon_name),
            )
        })
}

#[derive(Debug, Deserialize)]
pub struct GetWeaponsByCategoryRequest {
    pub category: Option<String>,
    pub weapon_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct GetWeaponsByCategoryResponse {
    pub weapons: Vec<Weapon>,
}

async fn get_weapons_by_category_handler(
    Json(request): Json<GetWeaponsByCategoryRequest>,
) -> Json<GetWeaponsByCategoryResponse> {
    let mut weapons = WeaponDatabase::all_weapons();

    if let Some(category) = &request.category {
        let category_filter = match category.as_str() {
            "Simple" => WeaponCategory::Simple,
            "Martial" => WeaponCategory::Martial,
            _ => {
                return Json(GetWeaponsByCategoryResponse { weapons });
            }
        };
        weapons.retain(|w| w.category == category_filter);
    }

    if let Some(weapon_type) = &request.weapon_type {
        let type_filter = match weapon_type.as_str() {
            "Melee" => WeaponType::Melee,
            "Ranged" => WeaponType::Ranged,
            _ => {
                return Json(GetWeaponsByCategoryResponse { weapons });
            }
        };
        weapons.retain(|w| w.weapon_type == type_filter);
    }

    Json(GetWeaponsByCategoryResponse { weapons })
}

// Spells handlers

#[derive(Debug, Serialize)]
pub struct ListSpellsResponse {
    pub spells: Vec<SpellInfo>,
}

#[derive(Debug, Serialize)]
pub struct SpellInfo {
    pub name: String,
    pub level: u8,
    pub school: String,
    pub classes: Vec<String>,
}

async fn list_spells_handler(
    State(state): State<AppState>,
) -> std::result::Result<Json<ListSpellsResponse>, (StatusCode, String)> {
    let db = state.spell_database.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database lock error: {}", e),
        )
    })?;

    let spells: Vec<SpellInfo> = db
        .list_spells()
        .into_iter()
        .map(|spell| SpellInfo {
            name: spell.name.clone(),
            level: spell.level.value(),
            school: spell.school.name().to_string(),
            classes: spell.classes.clone(),
        })
        .collect();

    Ok(Json(ListSpellsResponse { spells }))
}

async fn get_spell_handler(
    State(state): State<AppState>,
    axum::extract::Path(spell_name): axum::extract::Path<String>,
) -> std::result::Result<Json<Spell>, (StatusCode, String)> {
    let db = state.spell_database.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database lock error: {}", e),
        )
    })?;

    db.get_spell(&spell_name)
        .cloned()
        .map(Json)
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                format!("Spell not found: {}", spell_name),
            )
        })
}

#[derive(Debug, Deserialize)]
pub struct SearchSpellsRequest {
    pub query: String,
}

#[derive(Debug, Serialize)]
pub struct SearchSpellsResponse {
    pub spells: Vec<SpellInfo>,
}

async fn search_spells_handler(
    State(state): State<AppState>,
    Json(request): Json<SearchSpellsRequest>,
) -> std::result::Result<Json<SearchSpellsResponse>, (StatusCode, String)> {
    let db = state.spell_database.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database lock error: {}", e),
        )
    })?;

    let spells: Vec<SpellInfo> = db
        .search_spells(&request.query)
        .into_iter()
        .map(|spell| SpellInfo {
            name: spell.name.clone(),
            level: spell.level.value(),
            school: spell.school.name().to_string(),
            classes: spell.classes.clone(),
        })
        .collect();

    Ok(Json(SearchSpellsResponse { spells }))
}

#[derive(Debug, Deserialize)]
pub struct GetSpellsByLevelRequest {
    pub level: u8,
}

#[derive(Debug, Serialize)]
pub struct GetSpellsByLevelResponse {
    pub spells: Vec<SpellInfo>,
}

async fn get_spells_by_level_handler(
    State(state): State<AppState>,
    Json(request): Json<GetSpellsByLevelRequest>,
) -> std::result::Result<Json<GetSpellsByLevelResponse>, (StatusCode, String)> {
    if request.level > 9 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Spell level cannot exceed 9".to_string(),
        ));
    }

    let db = state.spell_database.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database lock error: {}", e),
        )
    })?;

    let spells: Vec<SpellInfo> = db
        .get_spells_by_level(request.level)
        .into_iter()
        .map(|spell| SpellInfo {
            name: spell.name.clone(),
            level: spell.level.value(),
            school: spell.school.name().to_string(),
            classes: spell.classes.clone(),
        })
        .collect();

    Ok(Json(GetSpellsByLevelResponse { spells }))
}

#[derive(Debug, Deserialize)]
pub struct GetSpellsBySchoolRequest {
    pub school: String,
}

#[derive(Debug, Serialize)]
pub struct GetSpellsBySchoolResponse {
    pub spells: Vec<SpellInfo>,
}

async fn get_spells_by_school_handler(
    State(state): State<AppState>,
    Json(request): Json<GetSpellsBySchoolRequest>,
) -> std::result::Result<Json<GetSpellsBySchoolResponse>, (StatusCode, String)> {
    let school = match request.school.to_lowercase().as_str() {
        "abjuration" => SpellSchool::Abjuration,
        "conjuration" => SpellSchool::Conjuration,
        "divination" => SpellSchool::Divination,
        "enchantment" => SpellSchool::Enchantment,
        "evocation" => SpellSchool::Evocation,
        "illusion" => SpellSchool::Illusion,
        "necromancy" => SpellSchool::Necromancy,
        "transmutation" => SpellSchool::Transmutation,
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid spell school: {}", request.school),
            ))
        }
    };

    let db = state.spell_database.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database lock error: {}", e),
        )
    })?;

    let spells: Vec<SpellInfo> = db
        .get_spells_by_school(school)
        .into_iter()
        .map(|spell| SpellInfo {
            name: spell.name.clone(),
            level: spell.level.value(),
            school: spell.school.name().to_string(),
            classes: spell.classes.clone(),
        })
        .collect();

    Ok(Json(GetSpellsBySchoolResponse { spells }))
}

#[derive(Debug, Deserialize)]
pub struct GetSpellsByClassRequest {
    pub class: String,
}

#[derive(Debug, Serialize)]
pub struct GetSpellsByClassResponse {
    pub spells: Vec<SpellInfo>,
}

async fn get_spells_by_class_handler(
    State(state): State<AppState>,
    Json(request): Json<GetSpellsByClassRequest>,
) -> std::result::Result<Json<GetSpellsByClassResponse>, (StatusCode, String)> {
    let db = state.spell_database.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database lock error: {}", e),
        )
    })?;

    let spells: Vec<SpellInfo> = db
        .get_spells_by_class(&request.class)
        .into_iter()
        .map(|spell| SpellInfo {
            name: spell.name.clone(),
            level: spell.level.value(),
            school: spell.school.name().to_string(),
            classes: spell.classes.clone(),
        })
        .collect();

    Ok(Json(GetSpellsByClassResponse { spells }))
}

async fn cast_spell_handler(
    State(state): State<AppState>,
    Json(request): Json<SpellCastRequest>,
) -> std::result::Result<Json<crate::spells::SpellCastResult>, (StatusCode, String)> {
    let db = state.spell_database.lock().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database lock error: {}", e),
        )
    })?;

    let spell = db.get_spell(&request.spell_name).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            format!("Spell not found: {}", request.spell_name),
        )
    })?;

    // Create temporary spell slots for validation
    let mut slots = crate::spells::SpellSlots::for_full_caster(request.caster_level).map_err(
        |e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to create spell slots: {}", e),
            )
        },
    )?;

    let result = state
        .spell_caster
        .cast(spell, &request, &mut slots)
        .map_err(|e| {
            (
                StatusCode::BAD_REQUEST,
                format!("Failed to cast spell: {}", e),
            )
        })?;

    Ok(Json(result))
}

#[derive(Debug, Deserialize)]
pub struct CreateFullCasterSlotsRequest {
    pub level: u8,
}

#[derive(Debug, Serialize)]
pub struct CreateFullCasterSlotsResponse {
    pub slots: std::collections::HashMap<u8, (u32, u32)>,
    pub max_level: u8,
}

async fn create_full_caster_slots_handler(
    Json(request): Json<CreateFullCasterSlotsRequest>,
) -> std::result::Result<Json<CreateFullCasterSlotsResponse>, (StatusCode, String)> {
    if request.level > 20 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Character level cannot exceed 20".to_string(),
        ));
    }

    let slots = crate::spells::SpellSlots::for_full_caster(request.level).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            format!("Failed to create spell slots: {}", e),
        )
    })?;

    Ok(Json(CreateFullCasterSlotsResponse {
        slots: slots.slots,
        max_level: slots.max_level,
    }))
}
