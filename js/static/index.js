import React from 'react';
import { render } from 'react-dom';
import App from '../components/app.js';

console.log("HELLO");
render(
    <App />,
    document.getElementById('app-root')
);