
import { Banner, BANNER_TYPE } from "./Banner";
import { filterUnitsByRarity } from "./Filter";

const gotSSR = (roll) => {
    return roll >= 91 && roll <= 101;
}

const gotSR = (roll) => {
    return roll >= 30 && roll <= 90;
}

const gotR = (roll) => {
    return roll >= 0 && roll <= 29;
}


const performRareSummon = (banner) =>
{
    const ssrUnits = filterUnitsByRarity("SSR");

    const roll = randint(0, 102);
}

