import * as axios from "axios";

export class API {
  session: axios.AxiosInstance;

  constructor(
    authorizationToken: string)
  {
    this.session = axios.default.create({
      headers: { Authorization: `Bearer ${authorizationToken}` }
    });
  }

  private async post(url: string, reqBody: object, headers: object = {}, responseType = "json" ): Promise<axios.AxiosResponse> {
    return this.session.post(url, reqBody, { headers: headers, responseType: responseType as any});
  }

  private async get(url: string, options: object = {}): Promise<axios.AxiosResponse> {
    return this.session.get(url, options);
  }

  private async head(url: string, options: object = {}): Promise<axios.AxiosResponse> {
    return this.session.head(url, options);
  }

  private async delete(url: string, options: object = {}): Promise<axios.AxiosResponse> {
    return this.session.delete(url, options);
  }
}

