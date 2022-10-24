import { STARTING_UNITS } from "./Database";
import Unit from "./Unit";


export class User
{
    rank : number;
    ds : number;
    party : Unit[][];
    box : Unit[];
    zeni : number;
    exp : number;
    boxSlots : number;
    constructor()
    {
        this.rank = 1;
        this.ds = 50;
        this.party = [
            STARTING_UNITS
        ];
        this.zeni = 1000;
        this.exp = 0;
        this.box = STARTING_UNITS;
        this.boxSlots = 100;
    }

}

// Methods do not work with redux, maybe because they can't be serialized?
// I don't know.
export const totalExpToRankUp = (user : User) : number => {
    return user.rank * 100;
}

export const canRankUp = (user : User) : boolean => {
    return user.exp >= totalExpToRankUp(user);
}

export const getExpPercentage = (user : User) : number =>{
    return user.exp/totalExpToRankUp(user);
}