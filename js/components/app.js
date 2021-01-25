import React from 'react';
import { Component } from 'react';
import SearchBox from './search_box';

// import rp from 'request-promise-native';


class App extends Component {
    render() {
        return <div id="app-frame">
            <div id="app-padding-left" />
            <SearchBox />
            <div id="app-padding-right" />
        </div>;
    }

}

export default App;