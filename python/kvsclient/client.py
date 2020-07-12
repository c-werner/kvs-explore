import requests
import json


def resp_get(resp, field):
    resp_content = resp.content.decode("utf-8")
    r = json.loads(resp_content)
    return r[field]


class KVS(object):
    def __init__(self, host):
        self.host = host

    def get(self, key, default=None):
        try:
            return self[key]
        except KeyError:
            return default

    def __getitem__(self, item):
        resp = requests.get('http://{host}/k/{key}'.format(
            host=self.host,
            key=item
        ))

        if resp.status_code == 404:
            raise KeyError

        return resp_get(resp, 'value')

    def set(self, key, value):
        requests.get('http://{host}/k/{key}?v={value}'.format(
            host=self.host,
            key=key,
            value=value,
        ))

    def __setitem__(self, key, value):
        return self.set(key, value)

    def has(self, key):
        resp = requests.get('http://{host}/h/{key}'.format(
            host=self.host,
            key=key,
        ))
        return resp_get(resp, 'ok')

    def __contains__(self, item):
        return self.has(item)

    def remove(self, key):
        resp = requests.get('http://{host}/d/{key}'.format(
            host=self.host,
            key=key,
        ))
        return resp_get(resp, 'ok')

    def __delitem__(self, key):
        return self.remove(key)

    def keys(self):
        resp = requests.get('http://{host}/list'.format(
            host=self.host,
        ))
        return resp_get(resp, 'keys')
