import { createSlice } from "@reduxjs/toolkit";
import { User } from "../../jsdependencies/User";

const initialData = () : User => {
    return new User();
}

/*
export const userReducer = createSlice({
    name: "user",
    initialState: {
        info: initialData()
    },
    reducers: {
        depositStones: (state, action) => {
        }

    }
});*/