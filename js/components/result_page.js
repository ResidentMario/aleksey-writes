import React from 'react';
import { Component } from 'react';
import ResultBlock from './result_block';
import { getResults } from '../functions/search';
class ResultPage extends Component {
    constructor() {
        super();
        const params = new URLSearchParams(window.location.search); 
        const query = params.get('text');
        this.state = {
            query: query,
            results: null,
            reached_es: null,
        }
    }

    onClickBack() {
        this.props.history.goBack();
    }

    fetch() {
        return getResults(this.state.query)
        .then(response => {
            response.json().then(results => {
                this.setState({...this.state, "results": results, "reached_es": true});
            })
        })
        .catch(_ => this.setState({...this.state, "reached_es": false}))
    }

    componentDidMount() {
        this.fetch();
    }

    render() {
        if (this.state.reached_es === false) {
            return <div id="result-page-frame">
                <div id="result-page-padding-left" />
                <div id="result-page-content-frame" style={{color: "red"}}>
                    Could not reach ElasticSearch. Please try again later. :(
                </div>
                <div id="result-page-padding-right" />
            </div>;
        }

        let resultsBlocks = [];
        if (this.state.results === null) {
            // ES query has not finished executing yet.
            resultsBlocks = [
                <ResultBlock placeholder={true} key={0}/>,
                <ResultBlock placeholder={true} key={1}/>,
                <ResultBlock placeholder={true} key={2}/>,
                <ResultBlock placeholder={true} key={3}/>,
                <ResultBlock placeholder={true} key={4}/>,
            ];
        } else if (this.state.results.hits.hits.length === 0) {
            // No hits, e.g. no matches.
            resultsBlocks = [<ResultBlock placeholder={true} key={0}/>];
        } else {
            resultsBlocks = this.state.results.hits.hits.map(
                (result, idx) => <ResultBlock result={result} placeholder={false} key={idx}/>
            );
        }

        return <div id="result-page-frame">
            <div id="result-page-padding-left" />
            <div id="result-page-content-frame">
                <div className="result-header-container">
                    <div className="back-button-icon" onClick={this.onClickBack.bind(this)}>
                        {/*
                            This SVG was generated in Inkscape. Aggravatingly, the translate
                            transform is original to the file that Inkscape creates,
                            even in plain SVG mode, and the offsets are plain wrong. I had to
                            redo the positioning of the elements by hand using the Inspector. I
                            guess Inkscape is a very poor fit for this use case. :(
                        */}
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                            <g transform="translate(-60,-135.5) scale(0.95 0.95)">
                                <path id="path16" d="m 72.098955,147.41071 -7.228795,7.2288 7.228795,7.22879" style={{fill: 'none', stroke: 'gray', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'miter', strokeMiterlimit: 4}} />
                                <path id="path18" d="M 64.87016,154.63951 H 84.0997" style={{fill: 'none', stroke: 'gray', strokeWidth: 2, strokeLinecap: 'round', strokeLinejoin: 'miter', strokeMiterlimit: 4}} />
                            </g>
                        </svg>
                    </div>
                    <div className="result-query">
                        {this.state.query}
                    </div>
                </div>
                <div className="result-blocks-frame">
                    {resultsBlocks}
                </div>
            </div>
            <div id="result-page-padding-right" />
        </div>

    }
}

export default ResultPage;