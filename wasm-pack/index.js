import('./pkg').then(module => {
  module.run()
  module.onLoad()
})