import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { User } from "../../tsdependencies/User";
import { MULTI_SUMMON_ROLL_COUNT, STONES_TO_MULTI_SUMMON, STONES_TO_SINGLE_SUMMON } from "../../tsdependencies/Constants";
import { Banner } from "../../tsdependencies/Banner";
import { performMultiSummon, performSingleSummon } from "../../tsdependencies/Summon";
import Unit from "../../tsdependencies/Unit";

const initialData = () : User => {
    return new User();
}


export const userSlice = createSlice({
    name: "user",
    initialState: initialData(),
    reducers: {
        depositStones: (state, action : PayloadAction<number>) => {
            return {
                ...state,
                ds: state.ds + action.payload
            };
        },
        
        singleSummon: (state, action : PayloadAction<Banner>) => {
            if (!(state.ds >= STONES_TO_SINGLE_SUMMON))
                return alert("You do not have enough dragon stones to perform a single summon.");
            if (state.box.length >= state.boxSlots)
                return alert("Your box is currently full. Please buy more slots or remove a unit in your box and try again.");
            const unitReceived : Unit = performSingleSummon(action.payload);
            return {
                ...state,
                ds: state.ds - STONES_TO_SINGLE_SUMMON,
                box: [...state.box, unitReceived]
            };
        },

        multiSummon: (state, action : PayloadAction<Banner>) => {
            if (!(state.ds >= STONES_TO_MULTI_SUMMON))
                return alert("You do not have enough dragon stones to perform a multi summon.");
            if (state.box.length + MULTI_SUMMON_ROLL_COUNT > state.boxSlots)
                return alert(`Your box is currently full. Please buy more slots or remove ${MULTI_SUMMON_ROLL_COUNT} units in your box and try again.`);
            const unitsReceived : Unit[] = performMultiSummon(action.payload);
            return {
                ...state,
                ds: state.ds - STONES_TO_MULTI_SUMMON,
                box: [...state.box, ...unitsReceived]
            };
        }
    }
});

export const {depositStones, singleSummon, multiSummon} = userSlice.actions;

export default userSlice.reducer;