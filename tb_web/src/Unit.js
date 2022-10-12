import { isValidRarity, isValidType, isValidURL } from "./Checker";
import { invalidRarityMessage, invalidTypeMessage, invalidURLMessage } from "./Error";


export class Unit
{
    /**
     * 
     * @param {String} url 
     * @param {String} icon should be a link to the icon
     * @param {String} art should be a link to the artwork
     * @param {String} name 
     * @param {String} rarity 
     * @param {String} type 
     * @param {String} leaderSkill 
     * @param {String} passiveSkill 
     * @param {String} superAtk 
     * @param {String[]} categories 
     * @param {Number} atk 
     * @param {Number} def 
     * @param {Number} hp 
     */
    constructor(url, icon, art,
                name, rarity, type,
                leaderSkill, passiveSkill,
                superAtk, categories,
                atk, def, hp) {
        
        if (!isValidURL(url))
            throw invalidURLMessage(url);
        if (!isValidURL(icon))
            throw invalidURLMessage(icon);
        if (!isValidURL(art))
            throw invalidURLMessage(art);
        if (!isValidRarity(rarity))
            throw invalidRarityMessage(rarity);
        if (!isValidType(type))
            throw invalidTypeMessage(type);
        if (!Number.isInteger(atk))
            throw `Invalid ATK stat: ${atk}`;
        if (!Number.isInteger(def))
            throw `Invalid DEF stat: ${def}`;
        if (!Number.isInteger(hp))
            throw `Invalid HP stat: ${hp}`;
            

        this.url = url;
        this.icon = icon;
        this.art = art;
        this.name = name;
        this.rarity = rarity;
        this.type = type;
        this.leaderSkill = leaderSkill;
        this.passiveSkill = passiveSkill;
        this.superAtk = superAtk;
        this.categories = categories;
        this.atk = atk;
        this.def = def;
        this.hp = hp;
    }
}