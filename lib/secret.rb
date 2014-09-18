
class SecretPageExtension < Middleman::Extension
  #option :enable_secret, true, 'Enable secret mode'
  #TODO: '/secret/{path}' をオプション化
  option :secret_link, '/secret/{path}', 'Path secret pages are generated at.'
  option :secret_tag, 'secret', 'Frontmatter tag for secret pages.'
  def manipulate_resource_list(resources)
    resources.map do |p|
      if p.data[options.secret_tag]
        #'secret/' + p.destination_path
        p.destination_path = Middleman::Blog::UriTemplates::apply_uri_template(
          Middleman::Blog::UriTemplates::uri_template(options.secret_link),
          path: p.destination_path
        )
      end
      p
    end
  end
end

::Middleman::Extensions.register(:secret, SecretPageExtension)

