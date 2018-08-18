using System;
using System.Linq;
using System.Net;
using System.Net.Http;
using System.Reflection;
using System.Threading.Tasks;

namespace DDNSUpdate
{
    class Program
    {
        internal static HttpClient httpClient = new HttpClient();

        static async Task Main(string[] args)
        {
            var providerName = Environment.GetEnvironmentVariable("DDNS_PROVIDER");
            var domainName = Environment.GetEnvironmentVariable("DDNS_DOMAIN");
            var recordName = Environment.GetEnvironmentVariable("DDNS_RECORD");
            var interval = int.Parse(Environment.GetEnvironmentVariable("DDNS_INTERVAL") ?? "600");

            if (providerName == null) {
                throw new ArgumentException("DDNS_PROVIDER must be set.");
            }

            if (domainName == null) {
                throw new ArgumentException("DDNS_DOMAIN must be set.");
            }

            if (recordName == null) {
                throw new ArgumentException("DDNS_RECORD must be set.");
            }

            var provider = GetProvider(providerName);

            while (true) {
                var publicIp = await GetPublicAddress();
                Console.WriteLine($"Current public IP: {publicIp}");

                await provider.Update(domainName, recordName, publicIp);

                Console.WriteLine($"Sleeping for {interval} seconds");
                await Task.Delay(interval * 1000);
            }
        }

        static Provider GetProvider(string providerName)
        {
            var matchingProviders = Assembly.GetEntryAssembly().DefinedTypes
                .Where(type => type.IsClass && !type.IsAbstract)
                .Where(type => typeof(Provider).GetTypeInfo().IsAssignableFrom(type))
                .Where(type => type.Name.ToLower() == providerName);

            if (!matchingProviders.Any()) {
                throw new ArgumentException("Unknown provider");
            }

            return Activator.CreateInstance(matchingProviders.First()) as Provider;
        }

        static async Task<IPAddress> GetPublicAddress()
        {
            var response = await httpClient.GetStringAsync("https://api.ipify.org");
            return IPAddress.Parse(response);
        }
    }
}
