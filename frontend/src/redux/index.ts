import { configureStore } from '@reduxjs/toolkit';
import { useDispatch, useSelector } from 'react-redux';

// reducers
import { appReducers } from './slices';
import { apiReducers } from './apiSlices';

const reduxStore = configureStore({
    reducer: appReducers,
    middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(apiReducers)
});

export default reduxStore;

export type RootState = ReturnType<typeof reduxStore.getState>;
export type AppDispatch = typeof reduxStore.dispatch;

export const useAppDispatch = useDispatch.withTypes<AppDispatch>();
export const useAppSelector = useSelector.withTypes<RootState>();