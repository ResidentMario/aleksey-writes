import React from 'react';
import { Component } from 'react';

function formatContentString(content_string) {
    let result = content_string.slice(0, 500);
    let last_space = result.lastIndexOf(" ");
    if (last_space === -1 || last_space === 0) {
        throw new Error("The given content string has no spaces, and is probably empty.")
    }
    result = result.slice(0, last_space);
    result = result + "..."
    return result;
}

function ResultBlock(props) {
    let document_type = props.placeholder ? "placeholder" : props.result._source.document_type;
    let logo = null;
    let themeColor = null;
    switch (document_type) {
        case "kaggle":
            logo = "kaggle-logo.png";
            themeColor = "#20beff";
            break;
        case "website":
            logo = "website-logo.png";
            themeColor = "#950000";
            break;
        case "medium":
            logo = "medium-logo.jpg";
            themeColor = "#14c867";
            break;
        case "spell":
            logo = "spell-logo.png";
            themeColor = "#002bab";
            break;
        case "placeholder":
            logo = "placeholder-logo.png"
            themeColor = "#888888";
            break;
        default:
            throw new Error(`Found document with illegal type ${document_type}.`)
    }
    logo = `/static/img/${logo}`;

    // _source has: backup, content, document_type, link, uid.
    const link = props.placeholder ? "#" : props.result._source.link;
    const backup = props.placeholder ? "#" : props.result._source.backup;
    // NOTE(aleksey): we display this no results found message both for queries that
    // haven't finished executing yet as well as for actual misses. This would be terrible UX in
    // a production product but hey, this ain't no such thing! /shrug
    const content = props.placeholder ? "No results found. :(" : formatContentString(props.result._source.content);
    const uid = props.placeholder ? "" : props.result._source.uid;
    return <div className="result-block" style={{"borderColor": themeColor}}>
        {/* <div className="result-block-header-spacer"/> */}
        <div className="result-block-header">
            <a className="result-block-header-href" style={{"color": themeColor}} href={link}>{uid}</a>
            <a className="result-block-raw-link-href" style={{"color": themeColor}} href={link}>{link}</a>
        </div>
        <div className="result-block-buttons">
            <a href={link}>
                <img className="result-block-logo" src={logo} />
            </a>
            <a href={backup}>
                <img className="result-block-logo" src="/static/img/archive-logo.png" />
            </a>
        </div>
        <div className="result-block-text">
            {content}
        </div>
    </div>;
}

export default ResultBlock;