import { configureStore } from "@reduxjs/toolkit";
import { userReducer } from "../reducers/UserReducer";

export default configureStore({
    reducer: {
        user: userReducer.reducer
    }
})