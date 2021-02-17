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
    switch (document_type) {
        case "kaggle":
            logo = "kaggle-logo.png";
            break;
        case "website":
            logo = "website-logo.png";
            break;
        case "medium":
            logo = "medium-logo.jpg";
            break;
        case "spell":
            logo = "spell-logo.png";
            break;
        case "placeholder":
            logo = "placeholder-logo.png"
            break;
        default:
            throw new Error(`Found document with illegal type ${document_type}.`)
    }
    logo = `/static/img/${logo}`;

    // console.log(props);
    // _source has: backup, content, document_type, link, uid.
    const link = props.placeholder ? "#" : props.result._source.link;
    const backup = props.placeholder ? "#" : props.result._source.backup;
    const content = props.placeholder ? "" : formatContentString(props.result._source.content);
    return <div className="result-block">
        <div className="result-block-header">
            <a href={link}>{link}</a>
        </div>
        <div className="result-block-text">
            {content}
        </div>
        <div className="result-block-buttons">
            <a href={link}>
                <img className="result-block-logo" src={logo} />
            </a>
            <a href={backup}>
                <img className="result-block-logo" src="/static/img/archive-logo.png" />
            </a>
        </div>
    </div>;
}

export default ResultBlock;