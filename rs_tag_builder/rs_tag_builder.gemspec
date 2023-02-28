# frozen_string_literal: true

require_relative "lib/rs_tag_builder/version"

Gem::Specification.new do |spec|
  spec.name = "rs_tag_builder"
  spec.version = RsTagBuilder::VERSION
  spec.authors = ["Kazuya Miyakawa"]
  spec.email = ["miyakawa.uro3@gmail.com"]

  spec.summary = "build tag"
  spec.description = "build tag"
  spec.homepage = "https://github.com/Uro3/rust_performance_test"
  spec.required_ruby_version = ">= 3.1.2"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
  spec.metadata["changelog_uri"] = spec.homepage

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  spec.files = Dir.chdir(__dir__) do
    `git ls-files -z`.split("\x0").reject do |f|
      (f == __FILE__) || f.match(%r{\A(?:(?:bin|test|spec|features)/|\.(?:git|travis|circleci)|appveyor)})
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]

  spec.extensions = ["ext/rs_tag_builder/extconf.rb"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"

  spec.add_dependency "rb_sys", "~> 0.9.39"
  spec.add_development_dependency "rake-compiler", "~> 1.2.0"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end