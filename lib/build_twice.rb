class BuildTwiceExtension < Middleman::Extension
  option :all, false, 'Build twice all resources.'
  option :frontmatter_str, "build_twice", 'Tag string on frontmatter.'

  helpers do
    def build_twice
      if @@second_time
        return yield if block_given?
      else
        @@need_build[current_resource.url] = current_resource
      end
      return ""
    end
  end

  def manipulate_resource_list(resources)
    for resource in resources
      if resource.data[options.frontmatter_str]
        @@need_build[resource.url] = resource
      end
    end
    resources
  end

  def after_configuration
    @@need_build = {}
    @@second_time = false
    @@options = {}
    @@options[:all] = options[:all]
    @@options[:frontmatter_str] = options[:frontmatter_str]

    options_hash = {
      glob: nil,
      dry_run: false,
      clean: false,
      parallel: true,
      only_changed: false,
      missing_and_changed: false,
      track_dependencies: true,
      visualize_graph: false,
      dependency_file: false
    }
    tmp = @app
    app.after_build do |build|
      unless @@second_time
        # p sitemap
        # p sitemap.methods
        # p sitemap.instance_variables
        @@second_time = true
        builder = Middleman::Builder.new(tmp, **options_hash)
        builder.run!
        # middleman-core/lib/middleman-core/cli/build.rb
        if @@options[:all]
          for resource in sitemap.resources
            next if resource.ignored?
            # Middleman::Cli::Build::build
            # Middleman::Cli::BuildAction.new(build).send(:build_resource, resource)
            # p builder.output_resource resource
          end
        else
          @@need_build.each do |url, resource|
            next unless resource
            next if resource.ignored?
            # Middleman::Builder.new(tmp, **options_hash).send(:output_resource, resource)
            # Middleman::Cli::Build::build
            # Middleman::Cli::BuildAction.new(build).send(:build_resource, resource)
            # p builder.output_resource resource
          end
        end
      end
    end
  end
# [:@callbacks, :@middleware, :@mappings, :@template_context_class, :@generic_template_context, :@config_context, :@config, :@extensions, :@sitemap]
# [:configure, :ignore, :rewrite_inline_urls, :callbacks_for, :subscribe_to_callbacks, :initialized, :before_extensions, :before_instance_block, :before_sitemap, :before_configuration, :after_configuration, :after_configuration_eval, :ready, :before_build, :after_build, :before_shutdown, :before, :before_render, :after_render, :before_server, :reload, :data, :execute_callbacks, :files, :"__contracts_ruby_original_development?_kb9i5uj18svu", :"__contracts_ruby_original_build?_kb9i5uj1ksqj", :__contracts_ruby_original_map_kb9i5uj18yru, :development?, :instrument, :mappings, :logger, :link_to, :image_tag, :asset_path, :set, :config, :inspect, :apply_cli_options, :use, :evaluate_configuration!, :extensions, :root_path, :prune_tilt_templates!, :mode?, :server?, :build?, :environment?, :production?, :to_s, :source_dir, :shutdown!, :functype, :environment, :root, :define_setting, :map, :activate, :Contract, :__contracts_ruby_original_config_context_kb9i5uize3pv, :config_context, :__contracts_ruby_original_config_kb9i5uiz3ka9, :__contracts_ruby_original_extensions_kb9i5uiz5lfi, :__contracts_ruby_original_sitemap_kb9i5uizia2o, :sitemap, :"__contracts_ruby_original_mode?_kb9i5uj0gnna", :generic_template_context, :template_context_class, :middleware, :__contracts_ruby_original_environment_kb9i5uj127em, :__contracts_ruby_original_mappings_kb9i5uizi817, :"__contracts_ruby_original_production?_kb9i5uj14e01", :"__contracts_ruby_original_server?_kb9i5uj0vwq", :__contracts_ruby_original_middleware_kb9i5uizblrh, :"__contracts_ruby_original_environment?_kb9i5uj13op6", :__contracts_ruby_original_source_dir_kb9i5uj11bcm, :present?, :deep_dup, :to_yaml, :duplicable?, :to_query, :untaint, :to_param, :__binding__, :acts_like?, :blank?, :presence, :pry, :is_haml?, :html_safe?, :try, :try!, :pretty_print_cycle, :pretty_print_inspect, :pretty_print_instance_variables, :pretty_print, :to_ruby, :to_v8, :to_json, :dup, :itself, :yield_self, :then, :taint, :tainted?, :untrust, :untrusted?, :trust, :frozen?, :methods, :singleton_methods, :protected_methods, :private_methods, :public_methods, :instance_variables, :instance_variable_get, :instance_variable_set, :instance_variable_defined?, :remove_instance_variable, :instance_of?, :kind_of?, :is_a?, :tap, :clone, :display, :hash, :class, :singleton_class, :public_send, :method, :public_method, :singleton_method, :class_eval, :define_singleton_method, :extend, :gem, :to_enum, :enum_for, :<=>, :pretty_inspect, :===, :=~, :!~, :nil?, :eql?, :respond_to?, :freeze, :object_id, :send, :suppress_warnings, :__send__, :!, :==, :!=, :equal?, :__id__, :instance_eval, :instance_exec]
#<Middleman::Application:0x16380>
#
  def initialize(app, options_hash={}, &block)
    super
  end
end

::Middleman::Extensions.register(:build_twice, BuildTwiceExtension)

