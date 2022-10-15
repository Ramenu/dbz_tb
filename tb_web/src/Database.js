import { readFileSync } from 'fs';
import Unit from './Unit.js';

export const N_UNITS = [
   "https://dbz-dokkanbattle.fandom.com/wiki/Assassin_in_the_Shadows_Yakon",
   "https://dbz-dokkanbattle.fandom.com/wiki/Earthborn_Warrior_Saibaiman_(AGL)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Infamous_Army_Red_Ribbon_Army_(AGL)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Lethal_Underling_Frieza_Soldier_(AGL)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Simple-Minded_Devil_Cell_Jr._(AGL)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Unarmed_Power_Pintar",
   "https://dbz-dokkanbattle.fandom.com/wiki/Warrior_from_Planet_Voon_Pui_Pui",
   "https://dbz-dokkanbattle.fandom.com/wiki/Army_of_Infamy_Red_Ribbon_Soldier",
   "https://dbz-dokkanbattle.fandom.com/wiki/Capable_in_Love_and_War_Jewel",
   "https://dbz-dokkanbattle.fandom.com/wiki/Deep_Sea_Guardsman_Pirate_Robot",
   "https://dbz-dokkanbattle.fandom.com/wiki/Earthborn_Warrior_Saibaiman_(TEQ)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Lethal_Underling_Frieza_Soldier_(TEQ)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Simple-Minded_Devil_Cell_Jr._(TEQ)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Earthborn_Warrior_Saibaiman_(INT)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Infamous_Army_Red_Ribbon_Army_(INT)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Lethal_Underling_Frieza_Soldier_(INT)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Loyal_Servant_Babidi%27s_Minion",
   "https://dbz-dokkanbattle.fandom.com/wiki/Simple-Minded_Devil_Cell_Jr._(INT)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Undisputed_Champion_Hercule",
   "https://dbz-dokkanbattle.fandom.com/wiki/Wicked_of_Heart_Yamu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Earthborn_Warrior_Saibaiman_(STR)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Fierce_Raider_Cui",
   "https://dbz-dokkanbattle.fandom.com/wiki/Infamous_Army_Red_Ribbon_Army_(STR)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Lethal_Underling_Frieza_Soldier_(STR)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Simple-Minded_Devil_Cell_Jr._(STR)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Slayer_of_Evil_Videl",
   "https://dbz-dokkanbattle.fandom.com/wiki/Steadfast_Soldier_Appule",
   "https://dbz-dokkanbattle.fandom.com/wiki/Well-Trained_Fist_Killa",
   "https://dbz-dokkanbattle.fandom.com/wiki/Earthborn_Warrior_Saibaiman_(PHY)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Infamous_Army_Red_Ribbon_Army_(PHY)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Lethal_Underling_Frieza_Soldier_(PHY)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Ominous_Will_Spopovich",
   "https://dbz-dokkanbattle.fandom.com/wiki/Simple-Minded_Devil_Cell_Jr._(PHY)"
];

export const R_UNITS = [
   "https://dbz-dokkanbattle.fandom.com/wiki/Fancy_Footwork_Android_18",
   "https://dbz-dokkanbattle.fandom.com/wiki/Free_at_Last_Android_17",
   "https://dbz-dokkanbattle.fandom.com/wiki/From_Hell_and_Back_Trunks_(Teen)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Light-Devouring_Beast_Yakon",
   "https://dbz-dokkanbattle.fandom.com/wiki/Menacing_Alien_Warrior_Raditz",
   "https://dbz-dokkanbattle.fandom.com/wiki/Paragon_of_Justice_Videl",
   "https://dbz-dokkanbattle.fandom.com/wiki/Professional_Tactician_Mercenary_Tao",
   "https://dbz-dokkanbattle.fandom.com/wiki/Speed_Impulse_Burter",
   "https://dbz-dokkanbattle.fandom.com/wiki/Terror%27s_Descent_Drum",
   "https://dbz-dokkanbattle.fandom.com/wiki/Unfaltering_Spirit_Tien",
   "https://dbz-dokkanbattle.fandom.com/wiki/Unfathomable_Speed_Burter",
   "https://dbz-dokkanbattle.fandom.com/wiki/Unwavering_Confidence_Yamcha",
   "https://dbz-dokkanbattle.fandom.com/wiki/Daring_Interception_Chiaotzu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Burdened_by_Destiny_Piccolo",
   "https://dbz-dokkanbattle.fandom.com/wiki/Lesson_in_Fear_Jeice",
   "https://dbz-dokkanbattle.fandom.com/wiki/Mad_Scientist%27s_Revenge_Dr._Gero",
   "https://dbz-dokkanbattle.fandom.com/wiki/Malicious_Scheme_Yamu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Namek%27s_Last_Warrior_Nail",
   "https://dbz-dokkanbattle.fandom.com/wiki/Nature-loving_Warrior_Android_16",
   "https://dbz-dokkanbattle.fandom.com/wiki/Savagery_Unleashed_Spopovich",
   "https://dbz-dokkanbattle.fandom.com/wiki/Space_Invader_Raditz",
   "https://dbz-dokkanbattle.fandom.com/wiki/The_One_and_Only_Yamcha",
   "https://dbz-dokkanbattle.fandom.com/wiki/Trickster_Chiaotzu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Well-Played_Trump_Card_Guldo",
   "https://dbz-dokkanbattle.fandom.com/wiki/Aloof_Warrior_Piccolo",
   "https://dbz-dokkanbattle.fandom.com/wiki/Champion%27s_Honor_Hercule",
   "https://dbz-dokkanbattle.fandom.com/wiki/Considerate_Captain_Captain_Ginyu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Craving_of_Battle_Cui",
   "https://dbz-dokkanbattle.fandom.com/wiki/Earnest_Passion_Chi-Chi",
   "https://dbz-dokkanbattle.fandom.com/wiki/Exquisite_Technique_Shen",
   "https://dbz-dokkanbattle.fandom.com/wiki/Facing_Fate_Nail",
   "https://dbz-dokkanbattle.fandom.com/wiki/Living_Weapon_Mercenary_Tao",
   "https://dbz-dokkanbattle.fandom.com/wiki/Precise_Action_Dr._Gero",
   "https://dbz-dokkanbattle.fandom.com/wiki/Steely_Determination_Gohan_(Kid)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Surging_Rage_Chi-Chi",
   "https://dbz-dokkanbattle.fandom.com/wiki/The_Timestopper_Guldo",
   "https://dbz-dokkanbattle.fandom.com/wiki/Unquenchable_Ambition_Machine_Pilaf",
   "https://dbz-dokkanbattle.fandom.com/wiki/Buried_Passion_Videl",
   "https://dbz-dokkanbattle.fandom.com/wiki/Captain%27s_Merit_Captain_Ginyu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Death_Sentence_Recoome",
   "https://dbz-dokkanbattle.fandom.com/wiki/Display_of_Strength_Zarbon",
   "https://dbz-dokkanbattle.fandom.com/wiki/Elite_Squadron%27s_Pride_Jeice",
   "https://dbz-dokkanbattle.fandom.com/wiki/Ghoulish_Burst_Tambourine",
   "https://dbz-dokkanbattle.fandom.com/wiki/Inevitable_Preemptive_Strike_Nappa",
   "https://dbz-dokkanbattle.fandom.com/wiki/Obstacle_Eliminator_Dodoria",
   "https://dbz-dokkanbattle.fandom.com/wiki/Passionate_Friendship_Krillin",
   "https://dbz-dokkanbattle.fandom.com/wiki/Target_Confirmed_Android_19",
   "https://dbz-dokkanbattle.fandom.com/wiki/Storm_of_Destruction_Dodoria",
   "https://dbz-dokkanbattle.fandom.com/wiki/A_Moment%27s_Chance_Yajirobe",
   "https://dbz-dokkanbattle.fandom.com/wiki/Cold_Analytics_Android_18",
   "https://dbz-dokkanbattle.fandom.com/wiki/Cold-blooded_Vandalism_Android_19",
   "https://dbz-dokkanbattle.fandom.com/wiki/Competitive_Comrades_Krillin",
   "https://dbz-dokkanbattle.fandom.com/wiki/Confident_Grin_Zarbon",
   "https://dbz-dokkanbattle.fandom.com/wiki/Cowardly_Savior_Yajirobe",
   "https://dbz-dokkanbattle.fandom.com/wiki/Dazzling_Destruction_Recoome",
   "https://dbz-dokkanbattle.fandom.com/wiki/Demon%27s_Disciple_Cymbal",
   "https://dbz-dokkanbattle.fandom.com/wiki/Devastating_Destruction_Nappa",
   "https://dbz-dokkanbattle.fandom.com/wiki/Escalating_Threat_Android_17",
   "https://dbz-dokkanbattle.fandom.com/wiki/Ironclad_Intentions_Android_16",
   "https://dbz-dokkanbattle.fandom.com/wiki/Killer_Instinct_Android_19",
   "https://dbz-dokkanbattle.fandom.com/wiki/Lightning_Onslaught_Tien",
   "https://dbz-dokkanbattle.fandom.com/wiki/Resistance_of_Tyranny_Gohan_(Kid)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Tiny_Terror_Cell_Jr."
];

export const SR_UNITS = [
   "https://dbz-dokkanbattle.fandom.com/wiki/Android_Evolution_Cell_(1st_Form)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Beautiful_but_Deadly_Android_18",
   "https://dbz-dokkanbattle.fandom.com/wiki/Dauntless_Runner_Android_17",
   "https://dbz-dokkanbattle.fandom.com/wiki/Determined_Defender_Goku",
   "https://dbz-dokkanbattle.fandom.com/wiki/Hawk_Eyes_Trunks_(Teen)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Indomitable_Human_Spirit_Yamcha",
   "https://dbz-dokkanbattle.fandom.com/wiki/Martial_Guidance_Master_Roshi",
   "https://dbz-dokkanbattle.fandom.com/wiki/Secret_Arts_Unleashed_Yamcha",
   "https://dbz-dokkanbattle.fandom.com/wiki/Speed_Tactics_Burter",
   "https://dbz-dokkanbattle.fandom.com/wiki/Stern_Teacher_Piccolo",
   "https://dbz-dokkanbattle.fandom.com/wiki/Unrivaled_Assassin_Mercenary_Tao",
   "https://dbz-dokkanbattle.fandom.com/wiki/Aura_Unleashed_Tien",
   "https://dbz-dokkanbattle.fandom.com/wiki/Blitz_Blade_Trunks_(Teen)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Fruits_of_Labor_Krillin",
   "https://dbz-dokkanbattle.fandom.com/wiki/Genius_of_War_Vegeta",
   "https://dbz-dokkanbattle.fandom.com/wiki/Infinite_Fighting_Spirit_Android_17",
   "https://dbz-dokkanbattle.fandom.com/wiki/Martial_Stability_Krillin",
   "https://dbz-dokkanbattle.fandom.com/wiki/Master_of_Magic_Guldo",
   "https://dbz-dokkanbattle.fandom.com/wiki/Peerless_Saiyan_Raditz",
   "https://dbz-dokkanbattle.fandom.com/wiki/Power_at_the_Eleventh_Hour_Chiaotzu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Time-traversing_Evil_Cell_(1st_Form)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Brilliant_Battle_Prowess_Captain_Ginyu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Call_of_Duty_Nail",
   "https://dbz-dokkanbattle.fandom.com/wiki/Crane_School%27s_Prestige_Master_Shen",
   "https://dbz-dokkanbattle.fandom.com/wiki/Cunning_Strategy_Dr._Gero",
   "https://dbz-dokkanbattle.fandom.com/wiki/Demonic_Pride_Piccolo",
   "https://dbz-dokkanbattle.fandom.com/wiki/Hero_Chosen_by_Earth_Hercule",
   "https://dbz-dokkanbattle.fandom.com/wiki/Hidden_Strength_Zarbon",
   "https://dbz-dokkanbattle.fandom.com/wiki/Honorable_Fighter_Captain_Ginyu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Messenger_from_the_Future_Trunks_(Teen)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Red_Ribbon_Army%27s_Grudge_Dr._Gero",
   "https://dbz-dokkanbattle.fandom.com/wiki/The_Gifted_One_Chiaotzu",
   "https://dbz-dokkanbattle.fandom.com/wiki/Alluring_Assassin_Android_18",
   "https://dbz-dokkanbattle.fandom.com/wiki/Blazing_Justice_Videl",
   "https://dbz-dokkanbattle.fandom.com/wiki/Despair%27s_Onslaught_Frieza_(1st_Form)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Elite_Warrior%27s_Rage_Nappa",
   "https://dbz-dokkanbattle.fandom.com/wiki/Emperor_of_Iniquity_Frieza_(1st_Form)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Heart_of_Innocence_Chi-Chi",
   "https://dbz-dokkanbattle.fandom.com/wiki/Imminent_Showdown_Cui",
   "https://dbz-dokkanbattle.fandom.com/wiki/Maximum_Firepower_Jeice",
   "https://dbz-dokkanbattle.fandom.com/wiki/Respect_to_the_Strong_Tien",
   "https://dbz-dokkanbattle.fandom.com/wiki/Secrets_of_the_Turtle_Style_Master_Roshi",
   "https://dbz-dokkanbattle.fandom.com/wiki/The_Saiyan_Among_Us_Goku",
   "https://dbz-dokkanbattle.fandom.com/wiki/The_Warrior_Awakens_Android_16",
   "https://dbz-dokkanbattle.fandom.com/wiki/Unrelenting_Fighter_Gohan_(Kid)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Calculated_Combat_Android_19",
   "https://dbz-dokkanbattle.fandom.com/wiki/Evil_Elegance_Zarbon",
   "https://dbz-dokkanbattle.fandom.com/wiki/Exploding_Rage_Gohan_(Kid)",
   "https://dbz-dokkanbattle.fandom.com/wiki/Extreme_Elite%27s_Pride_Vegeta",
   "https://dbz-dokkanbattle.fandom.com/wiki/Never-Ending_Assault_Recoome",
   "https://dbz-dokkanbattle.fandom.com/wiki/Power_Unleashed_Android_16",
   "https://dbz-dokkanbattle.fandom.com/wiki/Reckless_Rage_Dodoria",
   "https://dbz-dokkanbattle.fandom.com/wiki/Saiyan_Baptism_Nappa"
];

export const SSR_UNITS = [
   "https://dbz-dokkanbattle.fandom.com/wiki/Supreme_Warrior_Awakened_Super_Saiyan_Goku",
   "https://dbz-dokkanbattle.fandom.com/wiki/Pride_Regained_Super_Saiyan_Vegeta",
   "https://dbz-dokkanbattle.fandom.com/wiki/Convulsing_Rage_Super_Saiyan_Goku",
   "https://dbz-dokkanbattle.fandom.com/wiki/Super_Attack_Supreme_Super_Saiyan_Vegeta"
];

export const ALL_UNITS = () => {
   //const Unit = await import("./Unit");
   // First retrieve all JSON objects from file
   const json = () => {
      const fileName = "../../tb_scraper/units.json";
      try {
         const data = readFileSync(fileName, "utf-8");
         return JSON.parse(data);
      } catch (e) {
         console.error(`ERROR: ${e}`);
         return e;
      }
   }
   // Now map the unit's URL to the unit itself
   let allUnits = {};
   json().forEach(unit => {
      allUnits[unit.URL] = new Unit(
         unit["URL"],
         unit["Icon"],
         unit["Art"],
         unit["Name"],
         unit["Rarity"],
         unit["Type"],
         unit["Leader Skill"],
         unit["Passive Skill"],
         unit["Active Skill"],
         unit["Super attack"],
         unit["Ultra Super Attack"],
         unit["Unit Super Attack"],
         unit["Unit Super Attack Condition"],
         unit["Awakens Into"],
         unit["Categories"],
         unit["ATK"],
         unit["DEF"],
         unit["HP"]
      );
   });
   return allUnits;
}

