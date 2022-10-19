import { isValidRarity, isValidType, isValidURL } from "./Checker";
import { invalidRarityMessage, invalidTypeMessage, invalidURLMessage } from "./Error";


export default class Unit
{
    url : string;
    icon : string;
    art : string;
    name : string;
    rarity : string;
    type : string | undefined;
    leaderSkill : string;
    passiveSkill : string;
    activeSkill : string | undefined;
    superAtk : string;
    ultraSa : string | undefined;
    unitSa : string | undefined;
    unitSaCondition : string | undefined;
    awakensInto : string | undefined;
    categories : string[];
    atk : number;
    def : number;
    hp : number;

    constructor(url: string, 
                icon : string, 
                art : string,
                name : string, 
                rarity : string, 
                type : string | undefined,
                leaderSkill : string,
                passiveSkill : string,
                activeSkill : string | undefined, 
                superAtk : string, 
                ultraSa : string | undefined, 
                unitSa : string | undefined, 
                unitSaCondition : string | undefined,
                awakensInto : string | undefined, 
                categories : string[], 
                atk : number, 
                def : number, 
                hp : number) {
        
        if (!isValidURL(url))
            throw invalidURLMessage(url);
        if (!isValidURL(icon))
            throw invalidURLMessage(icon);
        if (!isValidURL(art))
            throw invalidURLMessage(art);
        if (awakensInto !== undefined) // This is allowed, as all objects may not have a dokkan awakening
            if (!isValidURL(awakensInto))
                throw invalidURLMessage(awakensInto);
        if (!isValidRarity(rarity))
            throw invalidRarityMessage(rarity);
        if (type !== undefined) // Some units (like selling statues) may not have a SUPER/EXTREME type
            if (!isValidType(type))
                throw invalidTypeMessage(type);
            
        this.url = url;
        this.icon = icon;
        this.art = art;
        this.name = name;
        this.rarity = rarity;
        this.type = type;
        this.leaderSkill = leaderSkill;
        this.passiveSkill = passiveSkill;
        this.activeSkill = activeSkill;
        this.superAtk = superAtk;
        this.ultraSa = ultraSa;
        this.unitSa = unitSa;
        this.unitSaCondition = unitSaCondition;
        this.awakensInto = awakensInto;
        this.categories = categories;
        this.atk = atk;
        this.def = def;
        this.hp = hp;
    }
}
