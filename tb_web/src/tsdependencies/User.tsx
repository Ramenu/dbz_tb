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
    }

    totalExpToRankUp() : number {
        return this.rank * 100;
    }

    canRankUp() : boolean {
        return this.exp >= this.totalExpToRankUp();
    }

    getExpPercentage() : number {
        return this.exp/this.totalExpToRankUp();
    }
}