import { STARTING_UNITS } from "./Database";
import Unit from "./Unit";


export class User
{
    rank : number;
    ds : number;
    party : Unit[][];
    zeni : number;
    constructor()
    {
        this.rank = 1;
        this.ds = 50;
        this.party = [
            STARTING_UNITS
        ];
        this.zeni = 1000;
    }
}