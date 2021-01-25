import React from 'react';
import { Component } from 'react';


class SearchBox extends Component {
    render() {
        return <div className="search-box-frame">
            <div className="some-jackass" title="What a guy, what a guy." />
            <div className="search-query-frame">
                <div className="magnifying-glass-icon">
                    <svg focusable="false" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path d="M15.5 14h-.79l-.28-.27A6.471 6.471 0 0 0 16 9.5 6.5 6.5 0 1 0 9.5 16c1.61 0 3.09-.59 4.23-1.57l.27.28v.79l5 4.99L20.49 19l-4.99-5zm-6 0C7.01 14 5 11.99 5 9.5S7.01 5 9.5 5 14 7.01 14 9.5 11.99 14 9.5 14z" fill="gray" />
                    </svg>
                </div>
                <input type="text" id="search-query" />
            </div>
            <div className="search-button-container">
                <input type="button" value="Search" className="search-button" />
                <input type="button" value="I'm Feeling Lucky" className="search-button" />
            </div>
        </div>
    }
}

export default SearchBox;