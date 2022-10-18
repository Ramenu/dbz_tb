import { STARTING_UNITS } from "./Database";


export class User
{
    constructor()
    {
        this.rank = 1;
        this.ds = 50;
        this.party = [
            STARTING_UNITS
        ];
    }
}