FROM ruby:2.7
RUN apt update && apt install -y vim
RUN gem install bundler

WORKDIR /tmp
COPY Gemfile Gemfile
COPY Gemfile.lock Gemfile.lock
RUN bundle install --jobs 4

