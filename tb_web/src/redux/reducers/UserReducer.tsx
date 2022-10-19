import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { User } from "../../tsdependencies/User";

const initialData = () : User => {
    return new User();
}


export const userReducer = createSlice({
    name: "user",
    initialState: {
        info: initialData()
    },
    reducers: {
        depositStones: (state, action : PayloadAction<number>) => {
            state.info.ds += action.payload;
        },
        withdrawStones: (state, action : PayloadAction<number>) => {
            state.info.ds += action.payload;
        }

    }
});


export default userReducer.reducer;