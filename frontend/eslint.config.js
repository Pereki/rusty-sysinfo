import vuetify from 'eslint-config-vuetify'

const config = await vuetify({
    ts: true,
})

export default [
    ...config,
    {
        rules: {
            '@stylistic/indent': ['error', 4],
            'vue/html-indent': 'off',
            'vue/script-indent': 'off',
        },
    },
]
