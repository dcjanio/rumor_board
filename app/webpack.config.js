const path = require('path');

module.exports = {
  webpack: (config, { buildId, dev, isServer, defaultLoaders, webpack }) => {
    // Loaders
    // If you had loaders in your old config, you can push them to config.module.rules
    // config.module.rules.push({ /* your loader config here */ });

    // Plugins
    // If you had plugins in your old config, you can push them to config.plugins
    // config.plugins.push(new webpack.SomePlugin());

    // Custom output directory (if you really need it, but usually not recommended for Next.js)
    // config.output.path = path.resolve(__dirname, 'dist');

    return config;
  },
}