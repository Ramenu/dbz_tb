import { createSlice } from "@reduxjs/toolkit";


export const prefsReducer = createSlice({
    name: "prefs",
    initialState: {
        toggleTopMenu: true
    },
    reducers: {

    }
});

export default prefsReducer.reducer;