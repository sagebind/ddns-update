using System;
using System.Linq;
using System.Net;
using System.Net.Sockets;
using System.Threading.Tasks;
using DigitalOcean.API;
using DigitalOcean.API.Models.Requests;

namespace DDNSUpdate
{
    public class DigitalOcean : Provider
    {
        const int DefaultTTL = 1800;
        private readonly DigitalOceanClient client =
            new DigitalOceanClient(Environment.GetEnvironmentVariable("DIGITALOCEAN_ACCESS_TOKEN"));

        public async Task Update(string domainName, string recordName, IPAddress ip)
        {
            var recordType = ip.AddressFamily == AddressFamily.InterNetworkV6 ? "AAAA" : "A";

            var record = (await client.DomainRecords.GetAll(domainName))
                .Where(r => r.Name == recordName)
                .Where(r => r.Type == recordType)
                .First();

            if (record != null)
            {
                if (record.Data != ip.ToString())
                {
                    await client.DomainRecords.Update(domainName, record.Id, new DomainRecord
                    {
                        Type = record.Type,
                        Name = record.Name,
                        Data = ip.ToString(),
                        Priority = record.Priority,
                        Port = record.Port,
                        TTL = record.TTL,
                        Weight = record.Weight,
                    });
                }
            }
            else
            {
                await client.DomainRecords.Create(domainName, new DomainRecord
                {
                    Type = recordType,
                    Name = recordName,
                    Data = ip.ToString(),
                    TTL = DefaultTTL,
                });
            }
        }
    }
}
