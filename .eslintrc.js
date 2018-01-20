module.exports = {
    "parserOptions": {
        "ecmaFeatures": {
            "jsx": true
        }
    },
    "env": {
        "browser": true,
        "jest": true
    },
    "extends": [
        "eslint:recommended",
        "airbnb",
        "prettier"
    ],
    "rules": {
        "linebreak-style": 0,
        "prettier/prettier": "error",
        "curly": ["error", "all"],
        "no-confusing-arrow": "error"
    },
    "plugins": [
        "prettier"
    ]
};
