k8s_yaml('infra/db.yaml')
include('./api/Tiltfile')
# include('./frontend/Tiltfile')

# A button to clear postgres database
load('ext://uibutton', 'cmd_button')
cmd_button('clear database',
		argv=['sh', '-c', 'rm -rf /var/lib/postgresql/data/*'],
		resource='dyson-db',
		icon_name='trash',
		text='clear',
)
