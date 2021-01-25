import React from 'react';
import SearchBox from './search_box';
import ResultPage from './result_page';
import { BrowserRouter as Router, Route, Switch, useHistory } from 'react-router-dom';

// import rp from 'request-promise-native';


function App() {
    return <Router>
        <Switch>
            <Route exact path="/" component={ SearchBox } />
            <Route path="/query" component={ ResultPage } />
        </Switch>
    </Router>;
}

export default App;