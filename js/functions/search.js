function getResults(query) {
    // NOTE(aleksey): we handle the case where the request fails, but not the case where the
    // request is timing out. Mostly because there is no built-in timeout support in the
    // native fetch API. Hopefully the service is well-behaved in this regard!
    const esUrl = process.env.ELASTICSEARCH_SERVICE_URL;
    return fetch(esUrl, {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        body: `{"query": {"match": {"content": {"query": "${query}"}}}}`
    })
}

export { getResults };