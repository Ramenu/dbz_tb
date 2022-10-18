import { invalidActionMessage, invalidNumberMessage } from "../../jsdependencies/Error";

const WITHDRAW_STONES = "WITHDRAW_STONES";
const DEPOSIT_STONES = "DEPOSIT_STONES";

export class StoneTransfer
{
    /**
     * 
     * @param {string} action 
     * @param {number} stones 
     */
    constructor(action, stones)
    {
        if (action !== WITHDRAW_STONES && action !== DEPOSIT_STONES)
            throw invalidActionMessage(action, [WITHDRAW_STONES, DEPOSIT_STONES]);
        if (!Number.isInteger(stones))
            throw invalidNumberMessage(stones);
        this.action = action;
        this.stones = stones;
    }
};