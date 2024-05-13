function generateApiKey(length) {
    const charset = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let key = '';

    for (let i = 0; i < length; i++) {
        const randomIndex = Math.floor(Math.random() * charset.length);
        key += charset.charAt(randomIndex);
    }

    return key;
}

module.exports = {
    generateApiKey
}